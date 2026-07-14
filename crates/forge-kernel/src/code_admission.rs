//! Bounded, non-executable code admission.

use serde::{Deserialize, Serialize};

use crate::{ActorKind, CandidateId, ForgeKernel, KernelError, ObjectId};

pub const MAX_CODE_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CodeAdmissionReceipt {
    pub source_id: String,
    pub relative_path: String,
    pub language: String,
    pub code_object: ObjectId,
    pub manifest_object: ObjectId,
    pub candidate: CandidateId,
    pub already_recorded: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CodePreview {
    pub candidate: CandidateId,
    pub relative_path: String,
    pub language: String,
    pub code: String,
    pub code_object: ObjectId,
}

#[derive(Deserialize, Serialize)]
struct CodeManifest<'a> {
    kind: String,
    version: u8,
    source_id: &'a str,
    relative_path: &'a str,
    language: &'a str,
    code_object: &'a str,
}

pub fn admit_pasted_code(
    kernel: &mut ForgeKernel,
    source_id: impl Into<String>,
    relative_path: impl Into<String>,
    language: impl Into<String>,
    code: &[u8],
) -> Result<CodeAdmissionReceipt, KernelError> {
    let source_id = source_id.into();
    let relative_path = relative_path.into();
    let language = language.into();
    validate(&source_id, &relative_path, &language, code)?;

    let code_object = ForgeKernel::object_id_for(code);
    let manifest = CodeManifest {
        kind: "forge-code-admission".into(),
        version: 1,
        source_id: &source_id,
        relative_path: &relative_path,
        language: &language,
        code_object: &code_object,
    };
    let manifest_bytes = serde_json::to_vec(&manifest)
        .map_err(|error| KernelError::Serialization(error.to_string()))?;
    let manifest_object = ForgeKernel::object_id_for(&manifest_bytes);
    let correlation_id = format!("code:{source_id}:{relative_path}");
    let already_recorded = kernel
        .candidate(&candidate_id_for(&manifest_object))
        .is_some();
    if !already_recorded {
        kernel.register_evidence(ActorKind::ImportedContent, code, correlation_id.clone())?;
        let recorded_manifest =
            kernel.register_evidence(ActorKind::ImportedContent, manifest_bytes, correlation_id)?;
        debug_assert_eq!(recorded_manifest, manifest_object);
    }
    let candidate = kernel.propose_candidate(&manifest_object, format!("code:{source_id}"))?;
    Ok(CodeAdmissionReceipt {
        source_id,
        relative_path,
        language,
        code_object,
        manifest_object,
        candidate,
        already_recorded,
    })
}

pub fn preview_code_candidate(
    kernel: &ForgeKernel,
    candidate_id: &str,
) -> Result<CodePreview, KernelError> {
    let candidate = kernel
        .candidate(candidate_id)
        .ok_or_else(|| KernelError::UnknownCandidate(candidate_id.into()))?;
    let manifest_bytes = &kernel
        .object(&candidate.evidence)
        .ok_or_else(|| KernelError::UnknownObject(candidate.evidence.clone()))?
        .bytes;
    let manifest: CodeManifest<'_> = serde_json::from_slice(manifest_bytes).map_err(|_| {
        KernelError::InvalidCodeAdmission("Candidate is not a code-admission manifest.".into())
    })?;
    if manifest.kind != "forge-code-admission" || manifest.version != 1 {
        return Err(KernelError::InvalidCodeAdmission(
            "Candidate is not a supported code manifest.".into(),
        ));
    }
    let code = kernel
        .object(&manifest.code_object)
        .ok_or_else(|| KernelError::UnknownObject(manifest.code_object.to_owned()))?;
    let code = String::from_utf8(code.bytes.clone()).map_err(|_| {
        KernelError::InvalidCodeAdmission("Code preview requires UTF-8 text.".into())
    })?;
    Ok(CodePreview {
        candidate: candidate_id.into(),
        relative_path: manifest.relative_path.to_owned(),
        language: manifest.language.to_owned(),
        code,
        code_object: manifest.code_object.to_owned(),
    })
}

fn candidate_id_for(manifest_object: &str) -> CandidateId {
    ForgeKernel::object_id_for(format!("candidate:v1:{manifest_object}").as_bytes())
}

fn validate(
    source_id: &str,
    relative_path: &str,
    language: &str,
    code: &[u8],
) -> Result<(), KernelError> {
    if source_id.trim().is_empty() || language.trim().is_empty() || code.is_empty() {
        return Err(KernelError::InvalidCodeAdmission(
            "Source ID, language, and code are required.".into(),
        ));
    }
    if code.len() > MAX_CODE_BYTES {
        return Err(KernelError::InvalidCodeAdmission(
            "Code exceeds the 1 MiB admission limit.".into(),
        ));
    }
    if !is_safe_repository_relative_path(relative_path) {
        return Err(KernelError::InvalidCodeAdmission(
            "Target path must be a safe repository-relative slash path.".into(),
        ));
    }
    Ok(())
}

pub(crate) fn is_safe_repository_relative_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    let has_drive_prefix = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    !path.is_empty()
        && !path.starts_with('/')
        && !path.starts_with('\\')
        && !has_drive_prefix
        && !path.contains('\\')
        && !path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CandidateState;

    #[test]
    fn code_is_evidence_backed_and_never_auto_approved() {
        let mut kernel = ForgeKernel::default();
        let receipt = admit_pasted_code(
            &mut kernel,
            "chat-a",
            "crates/demo.rs",
            "rust",
            b"fn main() {}",
        )
        .unwrap();
        assert_eq!(
            kernel.candidate(&receipt.candidate).unwrap().state,
            CandidateState::Proposed
        );
        assert_eq!(
            kernel.object(&receipt.code_object).unwrap().bytes,
            b"fn main() {}"
        );
    }

    #[test]
    fn duplicate_code_admission_is_idempotent_and_paths_are_bounded() {
        let mut kernel = ForgeKernel::default();
        let first = admit_pasted_code(
            &mut kernel,
            "chat-a",
            "src/demo.ts",
            "typescript",
            b"export {};",
        )
        .unwrap();
        let events = kernel.events().len();
        let second = admit_pasted_code(
            &mut kernel,
            "chat-a",
            "src/demo.ts",
            "typescript",
            b"export {};",
        )
        .unwrap();
        assert!(!first.already_recorded);
        assert!(second.already_recorded);
        assert_eq!(kernel.events().len(), events);
        for path in [
            "../escape.rs",
            "/absolute.rs",
            "C:/absolute.rs",
            "C:\\absolute.rs",
            "\\\\server\\share\\escape.rs",
        ] {
            assert!(matches!(
                admit_pasted_code(&mut kernel, "chat-a", path, "rust", b"x"),
                Err(KernelError::InvalidCodeAdmission(_))
            ));
        }
    }

    #[test]
    fn code_preview_recovers_exact_admitted_text_without_writing_anything() {
        let mut kernel = ForgeKernel::default();
        let receipt = admit_pasted_code(
            &mut kernel,
            "chat-a",
            "src/demo.rs",
            "rust",
            b"fn demo() {\n    println!(\"safe\");\n}",
        )
        .unwrap();
        let preview = preview_code_candidate(&kernel, &receipt.candidate).unwrap();
        assert_eq!(preview.relative_path, "src/demo.rs");
        assert!(preview.code.contains("println!"));
        assert_eq!(kernel.events().len(), 3);
    }
}
