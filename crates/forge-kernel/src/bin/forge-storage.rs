use std::{env, fs, path::PathBuf, process::ExitCode};

use forge_kernel::persistence::{
    PersistentForge, inventory_managed_workspace, preview_cache_cleanup,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("forge-storage: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    let root = PathBuf::from(args.next().ok_or_else(usage)?);
    let third = args.next();
    let value = match command.as_str() {
        "managed-inventory" => serde_json::to_value(
            inventory_managed_workspace(&root).map_err(|error| format!("{error:?}"))?,
        ),
        "cache-plan" => serde_json::to_value(
            preview_cache_cleanup(&root).map_err(|error| format!("{error:?}"))?,
        ),
        "compact-bootstrap" => {
            let database = PathBuf::from(third.ok_or_else(usage)?);
            let forge = PersistentForge::open(database).map_err(|error| format!("{error:?}"))?;
            let entry_count = forge.knowledge_record_count().map_err(|error| format!("{error:?}"))?;
            let inventory = inventory_managed_workspace(&root).map_err(|error| format!("{error:?}"))?;
            let bootstrap = root.join(".local/forge-bootstrap");
            fs::create_dir_all(&bootstrap).map_err(|error| error.to_string())?;
            let catalog = serde_json::json!({
                "schema_version": 4,
                "classifier_version": forge_kernel::knowledge::CLASSIFIER_VERSION,
                "entry_count": entry_count,
                "storage": "sqlite_fts5",
                "query": "tools/find-knowledge.ps1",
                "entries": []
            });
            fs::write(
                bootstrap.join("KNOWLEDGE_CATALOG.json"),
                serde_json::to_vec_pretty(&catalog).map_err(|error| error.to_string())?,
            ).map_err(|error| error.to_string())?;
            let binding = serde_json::json!({
                "canonical_root": root.to_string_lossy(),
                "inventory": inventory.files,
                "excluded_roots": inventory.excluded_roots,
                "root_digest": inventory.root_digest
            });
            fs::write(
                root.join(".local/forge-workspace-binding.json"),
                serde_json::to_vec_pretty(&binding).map_err(|error| error.to_string())?,
            ).map_err(|error| error.to_string())?;
            serde_json::to_value(serde_json::json!({
                "knowledge_catalog_bytes": fs::metadata(bootstrap.join("KNOWLEDGE_CATALOG.json")).map_err(|error| error.to_string())?.len(),
                "workspace_binding_bytes": fs::metadata(root.join(".local/forge-workspace-binding.json")).map_err(|error| error.to_string())?.len(),
                "entry_count": entry_count
            }))
        }
        _ => return Err(usage()),
    }
    .map_err(|error| error.to_string())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&value).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn usage() -> String {
    "usage: forge-storage <managed-inventory|cache-plan> <workspace-root> OR forge-storage compact-bootstrap <workspace-root> <forge.sqlite3>".into()
}
