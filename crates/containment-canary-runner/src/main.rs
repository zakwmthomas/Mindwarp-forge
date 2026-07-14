#[cfg(windows)]
mod windows_runner {
    use containment_canary_runner::{
        appcontainer_pe_flag, bounded_inventory, format_post_resume_exit, hex, parse_report,
        sha256, validate_image_load_flags, validate_lpac_observation,
    };
    use containment_profile::CONTRACT_VERSION;
    use std::{
        collections::{BTreeMap, BTreeSet},
        ffi::c_void,
        fs::{self, OpenOptions},
        io::Write,
        mem::{size_of, zeroed},
        net::TcpListener,
        path::{Path, PathBuf},
        ptr::{null, null_mut},
        thread,
        time::{Duration, Instant},
    };
    use windows_sys::Win32::Security::Isolation::{
        CreateAppContainerProfile, DeleteAppContainerProfile, GetAppContainerFolderPath,
    };
    use windows_sys::Win32::{
        Foundation::{
            CloseHandle, DBG_CONTINUE, DBG_EXCEPTION_NOT_HANDLED, ERROR_SUCCESS, GetLastError,
            HANDLE, INVALID_HANDLE_VALUE, LocalFree, NTSTATUS, WAIT_OBJECT_0, WAIT_TIMEOUT,
        },
        Security::{
            AccessCheck,
            Authorization::{
                ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
                EXPLICIT_ACCESS_W, GRANT_ACCESS, GetNamedSecurityInfoW, SDDL_REVISION_1,
                SE_FILE_OBJECT, SetEntriesInAclW, SetNamedSecurityInfoW, TRUSTEE_IS_SID,
                TRUSTEE_IS_UNKNOWN,
            },
            Cryptography::{BCRYPT_USE_SYSTEM_PREFERRED_RNG, BCryptGenRandom},
            DACL_SECURITY_INFORMATION, DuplicateToken, EqualSid, FreeSid, GENERIC_MAPPING,
            GetSecurityDescriptorControl, GetSidSubAuthority, GetSidSubAuthorityCount,
            GetTokenInformation, PRIVILEGE_SET, PROTECTED_DACL_SECURITY_INFORMATION, PSID,
            SE_DACL_PROTECTED, SECURITY_CAPABILITIES, SUB_CONTAINERS_AND_OBJECTS_INHERIT,
            SecurityImpersonation, TOKEN_APPCONTAINER_INFORMATION, TOKEN_DUPLICATE,
            TOKEN_MANDATORY_LABEL, TOKEN_QUERY, TokenAppContainerSid, TokenCapabilities,
            TokenElevation, TokenIntegrityLevel, TokenIsAppContainer,
            TokenIsLessPrivilegedAppContainer, UNPROTECTED_DACL_SECURITY_INFORMATION,
        },
        Storage::FileSystem::{FILE_GENERIC_EXECUTE, FILE_GENERIC_READ, GetFinalPathNameByHandleW},
        System::{
            Com::CoTaskMemFree,
            Diagnostics::Debug::{
                CREATE_PROCESS_DEBUG_EVENT, ContinueDebugEvent, DEBUG_EVENT,
                DebugActiveProcessStop, EXCEPTION_DEBUG_EVENT, EXIT_PROCESS_DEBUG_EVENT,
                LOAD_DLL_DEBUG_EVENT, WaitForDebugEvent,
            },
            IO::{CreateIoCompletionPort, GetQueuedCompletionStatus},
            JobObjects::{
                CreateJobObjectW, IsProcessInJob, JOB_OBJECT_CPU_RATE_CONTROL_ENABLE,
                JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP, JOB_OBJECT_LIMIT_ACTIVE_PROCESS,
                JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION, JOB_OBJECT_LIMIT_JOB_MEMORY,
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOB_OBJECT_LIMIT_PROCESS_MEMORY,
                JOB_OBJECT_LIMIT_PROCESS_TIME, JOBOBJECT_ASSOCIATE_COMPLETION_PORT,
                JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, JOBOBJECT_CPU_RATE_CONTROL_INFORMATION,
                JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectAssociateCompletionPortInformation,
                JobObjectBasicAccountingInformation, JobObjectCpuRateControlInformation,
                JobObjectExtendedLimitInformation, QueryInformationJobObject,
                SetInformationJobObject, TerminateJobObject,
            },
            SystemServices::{
                JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO, JOB_OBJECT_MSG_NEW_PROCESS, MAXIMUM_ALLOWED,
                PROCESS_MITIGATION_ASLR_POLICY, PROCESS_MITIGATION_DEP_POLICY,
                PROCESS_MITIGATION_DYNAMIC_CODE_POLICY,
                PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY,
                PROCESS_MITIGATION_IMAGE_LOAD_POLICY,
                PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY,
            },
            Threading::{
                CREATE_NO_WINDOW, CREATE_SUSPENDED, CREATE_UNICODE_ENVIRONMENT, CreateProcessW,
                DEBUG_ONLY_THIS_PROCESS, DeleteProcThreadAttributeList,
                EXTENDED_STARTUPINFO_PRESENT, GetCurrentProcess, GetExitCodeProcess,
                GetProcessMitigationPolicy, InitializeProcThreadAttributeList, OpenProcessToken,
                PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY,
                PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY, PROC_THREAD_ATTRIBUTE_JOB_LIST,
                PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY,
                PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, PROCESS_INFORMATION,
                ProcessASLRPolicy, ProcessDEPPolicy, ProcessDynamicCodePolicy,
                ProcessExtensionPointDisablePolicy, ProcessImageLoadPolicy,
                ProcessSystemCallDisablePolicy, ResumeThread, STARTUPINFOEXW,
                UpdateProcThreadAttribute, WaitForSingleObject,
            },
            WindowsProgramming::{
                PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT,
                PROCESS_CREATION_CHILD_PROCESS_RESTRICTED,
            },
        },
    };

    const FIXED_EXIT: u32 = 73;
    const LOW_INTEGRITY_RID: u32 = 0x1000;
    const WALL_TIMEOUT_MS: u32 = 3_000;
    const MEMORY_LIMIT: usize = 64 * 1024 * 1024;
    const MITIGATION_POLICY: u64 = 0x1
        | (1 << 8)
        | (1 << 12)
        | (1 << 16)
        | (1 << 20)
        | (1 << 24)
        | (1 << 28)
        | (1 << 32)
        | (1 << 36)
        | (1 << 52)
        | (1 << 56)
        | (1 << 60);

    fn last_error(label: &str) -> String {
        format!("{label}: {}", std::io::Error::last_os_error())
    }

    fn wide(value: &str) -> Result<Vec<u16>, String> {
        if value.contains('\0') {
            return Err("wide argument contains a forbidden character".into());
        }
        Ok(value.encode_utf16().chain([0]).collect())
    }

    unsafe fn pwstr_string(pointer: *const u16) -> Result<String, String> {
        if pointer.is_null() {
            return Err("null Windows string".into());
        }
        let mut length = 0usize;
        while length < 32_768 && unsafe { *pointer.add(length) } != 0 {
            length += 1;
        }
        if length == 32_768 {
            return Err("unterminated Windows string".into());
        }
        String::from_utf16(unsafe { std::slice::from_raw_parts(pointer, length) })
            .map_err(|_| "invalid Windows UTF-16".into())
    }

    #[derive(Debug)]
    struct OwnedHandle(HANDLE);
    impl OwnedHandle {
        fn new(handle: HANDLE, label: &str) -> Result<Self, String> {
            if handle.is_null() || handle == INVALID_HANDLE_VALUE {
                Err(last_error(label))
            } else {
                Ok(Self(handle))
            }
        }
    }
    impl Drop for OwnedHandle {
        fn drop(&mut self) {
            if !self.0.is_null() && self.0 != INVALID_HANDLE_VALUE {
                unsafe { CloseHandle(self.0) };
                self.0 = null_mut();
            }
        }
    }

    struct AttributeList {
        words: Vec<usize>,
    }
    impl AttributeList {
        fn new(count: u32) -> Result<Self, String> {
            let mut bytes = 0usize;
            unsafe { InitializeProcThreadAttributeList(null_mut(), count, 0, &mut bytes) };
            if bytes == 0 {
                return Err(last_error("attribute-list sizing"));
            }
            let mut words = vec![0usize; bytes.div_ceil(size_of::<usize>())];
            if unsafe {
                InitializeProcThreadAttributeList(words.as_mut_ptr().cast(), count, 0, &mut bytes)
            } == 0
            {
                return Err(last_error("attribute-list initialization"));
            }
            Ok(Self { words })
        }
        fn pointer(&mut self) -> *mut c_void {
            self.words.as_mut_ptr().cast()
        }
        fn set<T>(&mut self, attribute: u32, value: &T) -> Result<(), String> {
            if unsafe {
                UpdateProcThreadAttribute(
                    self.pointer(),
                    0,
                    attribute as usize,
                    (value as *const T).cast(),
                    size_of::<T>(),
                    null_mut(),
                    null(),
                )
            } == 0
            {
                Err(last_error("creation attribute"))
            } else {
                Ok(())
            }
        }
    }
    impl Drop for AttributeList {
        fn drop(&mut self) {
            unsafe { DeleteProcThreadAttributeList(self.pointer()) };
        }
    }

    struct AclGrant {
        path_wide: Vec<u16>,
        old_dacl: *mut windows_sys::Win32::Security::ACL,
        new_dacl: *mut windows_sys::Win32::Security::ACL,
        descriptor: windows_sys::Win32::Security::PSECURITY_DESCRIPTOR,
        original_acl: Vec<u8>,
        original_control: u16,
        restored: bool,
    }
    impl AclGrant {
        fn apply(path: &Path, sid: PSID) -> Result<Self, String> {
            let path_wide = wide(&path.to_string_lossy())?;
            let mut old_dacl = null_mut();
            let mut descriptor = null_mut();
            let status = unsafe {
                GetNamedSecurityInfoW(
                    path_wide.as_ptr(),
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION,
                    null_mut(),
                    null_mut(),
                    &mut old_dacl,
                    null_mut(),
                    &mut descriptor,
                )
            };
            if status != ERROR_SUCCESS {
                return Err(format!("read staging DACL: Windows error {status}"));
            }
            if old_dacl.is_null() {
                unsafe { LocalFree(descriptor) };
                return Err("staging directory has a null DACL".into());
            }
            let acl_size = unsafe { (*old_dacl).AclSize as usize };
            if acl_size < size_of::<windows_sys::Win32::Security::ACL>() || acl_size > 65_535 {
                unsafe { LocalFree(descriptor) };
                return Err("staging DACL size is invalid".into());
            }
            let original_acl =
                unsafe { std::slice::from_raw_parts(old_dacl.cast::<u8>(), acl_size) }.to_vec();
            let mut original_control = 0u16;
            let mut revision = 0u32;
            if unsafe {
                GetSecurityDescriptorControl(descriptor, &mut original_control, &mut revision)
            } == 0
            {
                unsafe { LocalFree(descriptor) };
                return Err(last_error("read staging security-descriptor control"));
            }
            let mut entry = EXPLICIT_ACCESS_W::default();
            entry.grfAccessPermissions = FILE_GENERIC_READ | FILE_GENERIC_EXECUTE;
            entry.grfAccessMode = GRANT_ACCESS;
            entry.grfInheritance = SUB_CONTAINERS_AND_OBJECTS_INHERIT;
            entry.Trustee.TrusteeForm = TRUSTEE_IS_SID;
            entry.Trustee.TrusteeType = TRUSTEE_IS_UNKNOWN;
            entry.Trustee.ptstrName = sid.cast();
            let mut new_dacl = null_mut();
            let status = unsafe { SetEntriesInAclW(1, &entry, old_dacl, &mut new_dacl) };
            if status != ERROR_SUCCESS {
                unsafe { LocalFree(descriptor) };
                return Err(format!("merge staging DACL: Windows error {status}"));
            }
            let status = unsafe {
                SetNamedSecurityInfoW(
                    path_wide.as_ptr() as *mut u16,
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION,
                    null_mut(),
                    null_mut(),
                    new_dacl,
                    null_mut(),
                )
            };
            if status != ERROR_SUCCESS {
                unsafe {
                    LocalFree(new_dacl.cast());
                    LocalFree(descriptor)
                };
                return Err(format!("apply staging DACL: Windows error {status}"));
            }
            Ok(Self {
                path_wide,
                old_dacl,
                new_dacl,
                descriptor,
                original_acl,
                original_control,
                restored: false,
            })
        }
        fn restore(&mut self) -> Result<(), String> {
            if self.restored {
                return Ok(());
            }
            let restore_info = DACL_SECURITY_INFORMATION
                | if self.original_control & SE_DACL_PROTECTED != 0 {
                    PROTECTED_DACL_SECURITY_INFORMATION
                } else {
                    UNPROTECTED_DACL_SECURITY_INFORMATION
                };
            let status = unsafe {
                SetNamedSecurityInfoW(
                    self.path_wide.as_mut_ptr(),
                    SE_FILE_OBJECT,
                    restore_info,
                    null_mut(),
                    null_mut(),
                    self.old_dacl,
                    null_mut(),
                )
            };
            if status != ERROR_SUCCESS {
                return Err(format!("restore staging DACL: Windows error {status}"));
            }
            let mut check_dacl = null_mut();
            let mut check_descriptor = null_mut();
            let status = unsafe {
                GetNamedSecurityInfoW(
                    self.path_wide.as_ptr(),
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION,
                    null_mut(),
                    null_mut(),
                    &mut check_dacl,
                    null_mut(),
                    &mut check_descriptor,
                )
            };
            if status != ERROR_SUCCESS || check_dacl.is_null() {
                if !check_descriptor.is_null() {
                    unsafe { LocalFree(check_descriptor) };
                }
                return Err(format!(
                    "verify restored staging DACL: Windows error {status}"
                ));
            }
            let check_size = unsafe { (*check_dacl).AclSize as usize };
            let check_bytes =
                unsafe { std::slice::from_raw_parts(check_dacl.cast::<u8>(), check_size) };
            let mut check_control = 0u16;
            let mut revision = 0u32;
            let control_ok = unsafe {
                GetSecurityDescriptorControl(check_descriptor, &mut check_control, &mut revision)
            } != 0;
            let identical = check_bytes == self.original_acl
                && control_ok
                && (check_control & SE_DACL_PROTECTED)
                    == (self.original_control & SE_DACL_PROTECTED);
            unsafe { LocalFree(check_descriptor) };
            if !identical {
                return Err("staging DACL restoration was not byte/control identical".into());
            }
            self.restored = true;
            Ok(())
        }
    }
    impl Drop for AclGrant {
        fn drop(&mut self) {
            let _ = self.restore();
            unsafe {
                LocalFree(self.new_dacl.cast());
                LocalFree(self.descriptor)
            };
        }
    }

    struct Resources {
        moniker: String,
        profile_created: bool,
        sid: PSID,
        profile_folder: Option<PathBuf>,
        profile_baseline: Option<BTreeSet<String>>,
        stage: Option<PathBuf>,
        sentinel_dir: Option<PathBuf>,
        acl: Option<AclGrant>,
        process: Option<OwnedHandle>,
        thread: Option<OwnedHandle>,
        job: Option<OwnedHandle>,
        completion: Option<OwnedHandle>,
        listener: Option<TcpListener>,
        debug_process_id: Option<u32>,
    }
    impl Resources {
        fn new(moniker: String) -> Self {
            Self {
                moniker,
                profile_created: false,
                sid: null_mut(),
                profile_folder: None,
                profile_baseline: None,
                stage: None,
                sentinel_dir: None,
                acl: None,
                process: None,
                thread: None,
                job: None,
                completion: None,
                listener: None,
                debug_process_id: None,
            }
        }
        fn stop_and_revoke(&mut self) -> Vec<String> {
            let mut errors = Vec::new();
            if let Some(process_id) = self.debug_process_id.take() {
                if unsafe { DebugActiveProcessStop(process_id) } == 0 {
                    errors.push(last_error("detach diagnostic observer during cleanup"));
                }
            }
            if let Some(job) = &self.job {
                unsafe { TerminateJobObject(job.0, 197) };
            }
            if let Some(process) = &self.process {
                let wait = unsafe { WaitForSingleObject(process.0, 2_000) };
                if wait != WAIT_OBJECT_0 {
                    errors.push(format!("process did not settle during cleanup: {wait}"));
                }
            }
            self.thread.take();
            self.process.take();
            self.job.take();
            self.completion.take();
            self.listener.take();
            if let Some(acl) = &mut self.acl {
                if let Err(e) = acl.restore() {
                    errors.push(e);
                }
            }
            self.acl.take();
            if let Some(stage) = &self.stage {
                if let Err(e) = remove_owned_tree(stage, "mindwarp-forge-lpac-stage-") {
                    errors.push(e);
                }
            }
            errors
        }
        fn final_cleanup(&mut self) -> Vec<String> {
            let mut errors = Vec::new();
            if let Some(dir) = &self.sentinel_dir {
                if let Err(e) = remove_owned_tree(dir, "mindwarp-forge-lpac-sentinel-") {
                    errors.push(e);
                }
            }
            if self.profile_created {
                let moniker = wide(&self.moniker).unwrap_or_default();
                let mut hr = unsafe { DeleteAppContainerProfile(moniker.as_ptr()) };
                if hr < 0 {
                    thread::sleep(Duration::from_millis(100));
                    hr = unsafe { DeleteAppContainerProfile(moniker.as_ptr()) };
                }
                if hr < 0 {
                    errors.push(format!("DeleteAppContainerProfile failed persistently: HRESULT 0x{:08x}; moniker={}",hr as u32,self.moniker));
                } else {
                    self.profile_created = false;
                }
            }
            if !self.sid.is_null() {
                unsafe { FreeSid(self.sid) };
                self.sid = null_mut();
            }
            errors
        }
    }
    impl Drop for Resources {
        fn drop(&mut self) {
            let _ = self.stop_and_revoke();
            let _ = self.final_cleanup();
        }
    }

    fn remove_owned_tree(path: &Path, prefix: &str) -> Result<(), String> {
        let temp = fs::canonicalize(std::env::temp_dir()).map_err(|e| e.to_string())?;
        let target = fs::canonicalize(path).map_err(|e| e.to_string())?;
        if target.parent() != Some(temp.as_path())
            || !target
                .file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with(prefix))
        {
            return Err(format!(
                "refused recursive cleanup outside owned temp root: {}",
                target.display()
            ));
        }
        fs::remove_dir_all(&target).map_err(|e| format!("remove {}: {e}", target.display()))
    }

    fn random_id() -> Result<String, String> {
        let mut bytes = [0u8; 16];
        let status = unsafe {
            BCryptGenRandom(
                null_mut(),
                bytes.as_mut_ptr(),
                bytes.len() as u32,
                BCRYPT_USE_SYSTEM_PREFERRED_RNG,
            )
        };
        if status < 0 {
            return Err(format!("BCryptGenRandom: NTSTATUS 0x{:08x}", status as u32));
        }
        Ok(hex(&bytes))
    }

    fn token_u32_os(token: HANDLE, class: i32) -> Result<u32, std::io::Error> {
        let mut value = 0u32;
        let mut returned = 0u32;
        if unsafe {
            GetTokenInformation(
                token,
                class,
                (&mut value as *mut u32).cast(),
                4,
                &mut returned,
            )
        } == 0
        {
            Err(std::io::Error::last_os_error())
        } else if returned != size_of::<u32>() as u32 {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("token DWORD query returned {returned} bytes"),
            ))
        } else {
            Ok(value)
        }
    }

    fn token_u32(token: HANDLE, class: i32, label: &str) -> Result<u32, String> {
        token_u32_os(token, class).map_err(|error| format!("{label}: {error}"))
    }

    fn lpac_access_mask(token: HANDLE) -> Result<u32, String> {
        let mut duplicate_raw = null_mut();
        if unsafe { DuplicateToken(token, SecurityImpersonation, &mut duplicate_raw) } == 0 {
            return Err(last_error("duplicate child token for LPAC access check"));
        }
        let duplicate = OwnedHandle::new(duplicate_raw, "LPAC access-check token")?;
        // A regular AppContainer receives bits 1 and 2; an LPAC must disregard
        // ALL_APPLICATION_PACKAGES and receive only bit 2.
        let sddl = wide("O:SYG:SYD:(A;;0x3;;;WD)(A;;0x1;;;AC)(A;;0x2;;;S-1-15-2-2)")?;
        let mut descriptor = null_mut();
        if unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(
                sddl.as_ptr(),
                SDDL_REVISION_1,
                &mut descriptor,
                null_mut(),
            )
        } == 0
        {
            return Err(last_error("build LPAC access discriminator"));
        }
        let mut mapping = GENERIC_MAPPING::default();
        let mut privileges = PRIVILEGE_SET::default();
        let mut privileges_len = size_of::<PRIVILEGE_SET>() as u32;
        let mut granted = 0u32;
        let mut access = 0;
        let checked = unsafe {
            AccessCheck(
                descriptor,
                duplicate.0,
                MAXIMUM_ALLOWED,
                &mut mapping,
                &mut privileges,
                &mut privileges_len,
                &mut granted,
                &mut access,
            )
        };
        let check_error = if checked == 0 {
            Some(std::io::Error::last_os_error())
        } else {
            None
        };
        unsafe { LocalFree(descriptor) };
        if let Some(error) = check_error {
            return Err(format!("LPAC access discriminator: {error}"));
        }
        if access == 0 {
            return Err("LPAC access discriminator denied every marker bit".into());
        }
        Ok(granted & 0x3)
    }

    fn token_buffer(token: HANDLE, class: i32) -> Result<Vec<usize>, String> {
        let mut bytes = 0u32;
        unsafe { GetTokenInformation(token, class, null_mut(), 0, &mut bytes) };
        if bytes == 0 {
            return Err(last_error("token buffer sizing"));
        }
        let mut words = vec![0usize; (bytes as usize).div_ceil(size_of::<usize>())];
        if unsafe {
            GetTokenInformation(token, class, words.as_mut_ptr().cast(), bytes, &mut bytes)
        } == 0
        {
            Err(last_error("token buffer"))
        } else {
            Ok(words)
        }
    }

    fn verify_parent_token() -> Result<(), String> {
        let mut raw = null_mut();
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut raw) } == 0 {
            return Err(last_error("parent token"));
        }
        let token = OwnedHandle::new(raw, "parent token")?;
        if token_u32(token.0, TokenElevation, "parent TokenElevation")? != 0 {
            return Err("runner refuses an elevated token".into());
        }
        if token_u32(token.0, TokenIsAppContainer, "parent TokenIsAppContainer")? != 0 {
            return Err("runner refuses to run from AppContainer".into());
        }
        Ok(())
    }

    fn sid_string(sid: PSID) -> Result<String, String> {
        let mut pointer = null_mut();
        if unsafe { ConvertSidToStringSidW(sid, &mut pointer) } == 0 {
            return Err(last_error("SID string"));
        }
        let result = unsafe { pwstr_string(pointer) };
        unsafe { LocalFree(pointer.cast()) };
        result
    }

    fn profile_folder(sid: PSID) -> Result<PathBuf, String> {
        let text = sid_string(sid)?;
        let sid_wide = wide(&text)?;
        let mut pointer = null_mut();
        let hr = unsafe { GetAppContainerFolderPath(sid_wide.as_ptr(), &mut pointer) };
        if hr < 0 {
            return Err(format!(
                "GetAppContainerFolderPath: HRESULT 0x{:08x}",
                hr as u32
            ));
        }
        let result = unsafe { pwstr_string(pointer) }.map(PathBuf::from);
        unsafe { CoTaskMemFree(pointer.cast()) };
        result
    }

    fn profile_entries(root: &Path) -> Result<BTreeSet<String>, String> {
        fn visit(root: &Path, current: &Path, out: &mut BTreeSet<String>) -> Result<(), String> {
            let mut entries = fs::read_dir(current)
                .map_err(|e| e.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;
            entries.sort_by_key(|e| e.file_name());
            for entry in entries {
                let path = entry.path();
                let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
                let rel = path
                    .strip_prefix(root)
                    .map_err(|e| e.to_string())?
                    .to_string_lossy()
                    .replace('\\', "/");
                if meta.file_type().is_symlink() {
                    return Err(format!("profile reparse/symlink: {rel}"));
                }
                out.insert(format!(
                    "{}:{rel}",
                    if meta.is_dir() {
                        "d"
                    } else if meta.is_file() {
                        "f"
                    } else {
                        "o"
                    }
                ));
                if meta.is_dir() {
                    visit(root, &path, out)?;
                }
            }
            Ok(())
        }
        let mut out = BTreeSet::new();
        visit(root, root, &mut out)?;
        Ok(out)
    }

    fn inventory_digest(inventory: &BTreeMap<String, [u8; 32]>) -> String {
        let mut bytes = Vec::new();
        for (path, hash) in inventory {
            bytes.extend_from_slice(path.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(hash);
        }
        hex(&sha256(&bytes))
    }

    fn query_job_active(job: HANDLE) -> Result<u32, String> {
        let mut info = JOBOBJECT_BASIC_ACCOUNTING_INFORMATION::default();
        if unsafe {
            QueryInformationJobObject(
                job,
                JobObjectBasicAccountingInformation,
                (&mut info as *mut JOBOBJECT_BASIC_ACCOUNTING_INFORMATION).cast(),
                size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>() as u32,
                null_mut(),
            )
        } == 0
        {
            Err(last_error("query job accounting"))
        } else {
            Ok(info.ActiveProcesses)
        }
    }

    fn wait_job_active(job: HANDLE, target: u32) -> Result<(), String> {
        for _ in 0..100 {
            if query_job_active(job)? == target {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(10));
        }
        Err(format!(
            "job active-process count did not settle to {target}"
        ))
    }

    fn verify_job_trace(completion: HANDLE) -> Result<(), String> {
        let mut new_processes = 0u32;
        let mut active_zero = false;
        for _ in 0..100 {
            loop {
                let mut message = 0u32;
                let mut key = 0usize;
                let mut overlapped = null_mut();
                let ok = unsafe {
                    GetQueuedCompletionStatus(
                        completion,
                        &mut message,
                        &mut key,
                        &mut overlapped,
                        0,
                    )
                };
                if ok == 0 {
                    let error = unsafe { GetLastError() };
                    if error == WAIT_TIMEOUT {
                        break;
                    }
                    return Err(format!("job completion read: Windows error {error}"));
                }
                if key != 1 {
                    return Err("job completion key mismatch".into());
                }
                if message == JOB_OBJECT_MSG_NEW_PROCESS {
                    new_processes += 1;
                }
                if message == JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO {
                    active_zero = true;
                }
            }
            if active_zero {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        if !active_zero {
            return Err("job completion stream lacks active-process-zero".into());
        }
        if new_processes != 1 {
            return Err(format!(
                "job completion stream observed {new_processes} processes, expected exactly one"
            ));
        }
        Ok(())
    }

    unsafe fn mitigation_flags<T: Default>(process: HANDLE, policy: i32) -> Result<u32, String> {
        let mut value = T::default();
        if unsafe {
            GetProcessMitigationPolicy(
                process,
                policy,
                (&mut value as *mut T).cast(),
                size_of::<T>(),
            )
        } == 0
        {
            return Err(last_error("query mitigation"));
        }
        Ok(unsafe { *(std::ptr::addr_of!(value) as *const u32) })
    }

    fn verify_suspended(process: HANDLE, job: HANDLE, sid: PSID) -> Result<String, String> {
        let mut token_raw = null_mut();
        if unsafe { OpenProcessToken(process, TOKEN_QUERY | TOKEN_DUPLICATE, &mut token_raw) } == 0
        {
            return Err(last_error("child token"));
        }
        let token = OwnedHandle::new(token_raw, "child token")?;
        if token_u32(token.0, TokenIsAppContainer, "child TokenIsAppContainer")? != 1 {
            return Err("child token is not AppContainer".into());
        }
        let class_46 = token_u32_os(token.0, TokenIsLessPrivilegedAppContainer)
            .map_err(|error| error.raw_os_error().unwrap_or(0));
        let lpac_verification = validate_lpac_observation(class_46, lpac_access_mask(token.0)?)?;
        let app = token_buffer(token.0, TokenAppContainerSid)?;
        let info = unsafe { &*(app.as_ptr().cast::<TOKEN_APPCONTAINER_INFORMATION>()) };
        if info.TokenAppContainer.is_null() || unsafe { EqualSid(info.TokenAppContainer, sid) } == 0
        {
            return Err("child AppContainer SID mismatch".into());
        }
        let caps = token_buffer(token.0, TokenCapabilities)?;
        if unsafe { *(caps.as_ptr().cast::<u32>()) } != 0 {
            return Err("child capability count is nonzero".into());
        }
        let integrity = token_buffer(token.0, TokenIntegrityLevel)?;
        let label = unsafe { &*(integrity.as_ptr().cast::<TOKEN_MANDATORY_LABEL>()) };
        let count = unsafe { *GetSidSubAuthorityCount(label.Label.Sid) } as u32;
        if count == 0 {
            return Err("integrity SID has no RID".into());
        }
        let rid = unsafe { *GetSidSubAuthority(label.Label.Sid, count - 1) };
        if rid != LOW_INTEGRITY_RID {
            return Err(format!("child integrity RID is {rid}, expected Low"));
        }
        let mut in_job = 0;
        if unsafe { IsProcessInJob(process, job, &mut in_job) } == 0 || in_job == 0 {
            return Err("child is not atomically assigned to expected job".into());
        }
        if query_job_active(job)? != 1 {
            return Err("job active-process count is not exactly one".into());
        }
        unsafe {
            if mitigation_flags::<PROCESS_MITIGATION_DEP_POLICY>(process, ProcessDEPPolicy)? & 1
                == 0
            {
                return Err("DEP mitigation missing".into());
            }
            if mitigation_flags::<PROCESS_MITIGATION_ASLR_POLICY>(process, ProcessASLRPolicy)? & 7
                != 7
            {
                return Err("ASLR mitigations missing".into());
            }
            if mitigation_flags::<PROCESS_MITIGATION_DYNAMIC_CODE_POLICY>(
                process,
                ProcessDynamicCodePolicy,
            )? & 1
                == 0
            {
                return Err("dynamic-code mitigation missing".into());
            }
            if mitigation_flags::<PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY>(
                process,
                ProcessExtensionPointDisablePolicy,
            )? & 1
                == 0
            {
                return Err("extension-point mitigation missing".into());
            }
            if mitigation_flags::<PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY>(
                process,
                ProcessSystemCallDisablePolicy,
            )? & 1
                == 0
            {
                return Err("Win32k mitigation missing".into());
            }
            validate_image_load_flags(mitigation_flags::<PROCESS_MITIGATION_IMAGE_LOAD_POLICY>(
                process,
                ProcessImageLoadPolicy,
            )?)?;
        }
        Ok(lpac_verification.into())
    }

    struct PassEvidence {
        sid: String,
        binary_hash: String,
        source_hash: String,
        contract_hash: String,
        sentinel_hash: String,
        repository_hash: String,
        report_hash: String,
        exit_code: u32,
        lpac_verification: String,
    }

    #[derive(Debug, Default)]
    struct DebugEventEvidence {
        sequence: usize,
        kind: &'static str,
        path: Option<String>,
        base: Option<usize>,
        exception_code: Option<u32>,
        exception_address: Option<usize>,
        first_chance: Option<u32>,
        exit_code: Option<u32>,
    }

    #[derive(Debug, Default)]
    struct DebugTrace {
        candidate_sha256: String,
        lpac_verification: String,
        events: Vec<DebugEventEvidence>,
        exit_code: Option<u32>,
    }

    fn event_file_path_and_close(handle: HANDLE) -> Result<Option<String>, String> {
        if handle.is_null() || handle == INVALID_HANDLE_VALUE {
            return Ok(None);
        }
        let result = (|| {
            let mut buffer = vec![0u16; 32_768];
            let length = unsafe {
                GetFinalPathNameByHandleW(handle, buffer.as_mut_ptr(), buffer.len() as u32, 0)
            };
            if length == 0 {
                return Err(last_error("debug-event final path"));
            }
            if length as usize >= buffer.len() {
                return Err("debug-event final path exceeds fixed bound".into());
            }
            String::from_utf16(&buffer[..length as usize])
                .map(Some)
                .map_err(|_| "debug-event path is invalid UTF-16".into())
        })();
        unsafe { CloseHandle(handle) };
        result
    }

    fn exception_continue_status(code: u32) -> NTSTATUS {
        if code == 0x8000_0003 {
            DBG_CONTINUE
        } else {
            DBG_EXCEPTION_NOT_HANDLED
        }
    }

    fn wait_and_record_debug_event(
        trace: &mut DebugTrace,
        timeout_ms: u32,
    ) -> Result<bool, String> {
        if trace.events.len() >= 256 {
            return Err("debug-event count exceeds fixed bound".into());
        }
        let mut event = DEBUG_EVENT::default();
        if unsafe { WaitForDebugEvent(&mut event, timeout_ms) } == 0 {
            return Err(last_error("wait for debug event"));
        }
        let mut evidence = DebugEventEvidence {
            sequence: trace.events.len(),
            ..Default::default()
        };
        let mut continue_status = DBG_CONTINUE;
        let mut exited = false;
        let extraction = (|| -> Result<(), String> {
            match event.dwDebugEventCode {
                CREATE_PROCESS_DEBUG_EVENT => {
                    evidence.kind = "create_process";
                    let info = unsafe { event.u.CreateProcessInfo };
                    evidence.path = event_file_path_and_close(info.hFile)?;
                    evidence.base = Some(info.lpBaseOfImage as usize);
                }
                LOAD_DLL_DEBUG_EVENT => {
                    evidence.kind = "load_dll";
                    let info = unsafe { event.u.LoadDll };
                    evidence.path = event_file_path_and_close(info.hFile)?;
                    evidence.base = Some(info.lpBaseOfDll as usize);
                }
                EXCEPTION_DEBUG_EVENT => {
                    evidence.kind = "exception";
                    let info = unsafe { event.u.Exception };
                    let code = info.ExceptionRecord.ExceptionCode as u32;
                    evidence.exception_code = Some(code);
                    evidence.exception_address =
                        Some(info.ExceptionRecord.ExceptionAddress as usize);
                    evidence.first_chance = Some(info.dwFirstChance);
                    continue_status = exception_continue_status(code);
                }
                EXIT_PROCESS_DEBUG_EVENT => {
                    evidence.kind = "exit_process";
                    let code = unsafe { event.u.ExitProcess.dwExitCode };
                    evidence.exit_code = Some(code);
                    trace.exit_code = Some(code);
                    exited = true;
                }
                2 => evidence.kind = "create_thread",
                4 => evidence.kind = "exit_thread",
                7 => evidence.kind = "unload_dll",
                8 => evidence.kind = "debug_string_ignored",
                9 => evidence.kind = "rip",
                other => return Err(format!("unknown debug-event code {other}")),
            }
            Ok(())
        })();
        let continued =
            unsafe { ContinueDebugEvent(event.dwProcessId, event.dwThreadId, continue_status) };
        trace.events.push(evidence);
        if continued == 0 {
            return Err(last_error("continue debug event"));
        }
        extraction?;
        Ok(exited)
    }

    fn drive_debug_events(trace: &mut DebugTrace) -> Result<u32, String> {
        let started = Instant::now();
        loop {
            let elapsed = started.elapsed();
            if elapsed >= Duration::from_millis(WALL_TIMEOUT_MS as u64) {
                return Err("debug event wall timeout".into());
            }
            let remaining = Duration::from_millis(WALL_TIMEOUT_MS as u64) - elapsed;
            let timeout = u32::try_from(remaining.as_millis().max(1)).unwrap_or(WALL_TIMEOUT_MS);
            let first = trace.events.is_empty();
            if wait_and_record_debug_event(trace, timeout)? {
                return trace
                    .exit_code
                    .ok_or_else(|| "exit debug event omitted its status".into());
            }
            if first && trace.events.first().map(|event| event.kind) != Some("create_process") {
                return Err("first debug event was not create-process".into());
            }
        }
    }

    fn debug_trace_json(trace: &DebugTrace) -> String {
        let events = trace
            .events
            .iter()
            .map(|event| {
                let mut fields = vec![
                    format!("\"sequence\":{}", event.sequence),
                    format!("\"kind\":\"{}\"", event.kind),
                ];
                if let Some(path) = &event.path {
                    fields.push(format!("\"path\":\"{}\"", json_escape(path)));
                }
                if let Some(base) = event.base {
                    fields.push(format!("\"base\":\"0x{base:016X}\""));
                }
                if let Some(code) = event.exception_code {
                    fields.push(format!("\"exception_code\":\"0x{code:08X}\""));
                }
                if let Some(address) = event.exception_address {
                    fields.push(format!("\"exception_address\":\"0x{address:016X}\""));
                }
                if let Some(first) = event.first_chance {
                    fields.push(format!("\"first_chance\":{first}"));
                }
                if let Some(code) = event.exit_code {
                    fields.push(format!("\"exit_code\":\"0x{code:08X}\""));
                }
                format!("{{{}}}", fields.join(","))
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "\"diagnostic\":{{\"debug_semantics_changed\":true,\"candidate_sha256\":\"{}\",\"lpac_verification\":\"{}\",\"event_count\":{},\"events\":[{}],\"denial_proved\":false,\"runtime_cause_proved\":false,\"profile_created\":true,\"capability_added\":false,\"registry_modified\":false}}",
            trace.candidate_sha256,
            json_escape(&trace.lpac_verification),
            trace.events.len(),
            events
        )
    }

    fn trial(
        canary: &Path,
        repo: &Path,
        diagnostic: bool,
    ) -> (
        String,
        String,
        Result<PassEvidence, String>,
        Vec<String>,
        Option<DebugTrace>,
    ) {
        let run_id = match random_id() {
            Ok(id) => id,
            Err(e) => return (String::new(), String::new(), Err(e), vec![], None),
        };
        let moniker = format!("MindwarpForge.P7b1b.{run_id}");
        let mut r = Resources::new(moniker.clone());
        let mut debug_trace = diagnostic.then(DebugTrace::default);
        let outcome = (|| -> Result<PassEvidence, String> {
            verify_parent_token()?;
            let canary_bytes = fs::read(canary).map_err(|e| format!("read canary: {e}"))?;
            if !appcontainer_pe_flag(&canary_bytes)? {
                return Err("canary lacks PE AppContainer flag".into());
            }
            let binary_hash = hex(&sha256(&canary_bytes));
            if let Some(trace) = &mut debug_trace {
                trace.candidate_sha256 = binary_hash.clone();
            }
            let source_hash = hex(&sha256(
                &fs::read(repo.join("crates/containment-denial-canary/src/main.rs"))
                    .map_err(|e| e.to_string())?,
            ));
            let contract_hash = hex(&sha256(
                &fs::read(repo.join("contracts/containment-profile-contract.md"))
                    .map_err(|e| e.to_string())?,
            ));
            if CONTRACT_VERSION != 1 {
                return Err("containment contract version drift".into());
            }
            let before = bounded_inventory(repo)?;
            let repository_hash = inventory_digest(&before);
            let temp = fs::canonicalize(std::env::temp_dir()).map_err(|e| e.to_string())?;
            let stage = temp.join(format!("mindwarp-forge-lpac-stage-{run_id}"));
            let sentinel_dir = temp.join(format!("mindwarp-forge-lpac-sentinel-{run_id}"));
            fs::create_dir(&stage).map_err(|e| e.to_string())?;
            r.stage = Some(stage.clone());
            fs::create_dir(&sentinel_dir).map_err(|e| e.to_string())?;
            r.sentinel_dir = Some(sentinel_dir.clone());
            let staged = stage.join("containment-denial-canary.exe");
            fs::copy(canary, &staged).map_err(|e| e.to_string())?;
            if sha256(&fs::read(&staged).map_err(|e| e.to_string())?) != sha256(&canary_bytes) {
                return Err("staged canary hash mismatch".into());
            }
            let sentinel = sentinel_dir.join("synthetic-denied-sentinel.bin");
            let sentinel_bytes = b"Mindwarp Forge synthetic denial sentinel v1\r\n";
            fs::write(&sentinel, sentinel_bytes).map_err(|e| e.to_string())?;
            let sentinel_hash = hex(&sha256(sentinel_bytes));
            let m = wide(&moniker)?;
            let display = wide("Mindwarp Forge P7b-1b denial canary")?;
            let description = wide("One-run synthetic zero-capability LPAC denial proof")?;
            let mut sid = null_mut();
            let hr = unsafe {
                CreateAppContainerProfile(
                    m.as_ptr(),
                    display.as_ptr(),
                    description.as_ptr(),
                    null(),
                    0,
                    &mut sid,
                )
            };
            if hr < 0 {
                return Err(format!(
                    "CreateAppContainerProfile failed/collided: HRESULT 0x{:08x}",
                    hr as u32
                ));
            }
            r.profile_created = true;
            r.sid = sid;
            let sid_text = sid_string(sid)?;
            let profile = profile_folder(sid)?;
            r.profile_folder = Some(profile.clone());
            let baseline = profile_entries(&profile)?;
            r.profile_baseline = Some(baseline);
            let report = profile.join(format!("forge-denial-report-{run_id}.json"));
            if report.exists() {
                return Err("report path preexists".into());
            }
            r.acl = Some(AclGrant::apply(&stage, sid)?);
            let job = OwnedHandle::new(unsafe { CreateJobObjectW(null(), null()) }, "create job")?;
            let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
                | JOB_OBJECT_LIMIT_ACTIVE_PROCESS
                | JOB_OBJECT_LIMIT_PROCESS_MEMORY
                | JOB_OBJECT_LIMIT_JOB_MEMORY
                | JOB_OBJECT_LIMIT_PROCESS_TIME
                | JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION;
            limits.BasicLimitInformation.ActiveProcessLimit = 1;
            limits.BasicLimitInformation.PerProcessUserTimeLimit = 20_000_000;
            limits.ProcessMemoryLimit = MEMORY_LIMIT;
            limits.JobMemoryLimit = MEMORY_LIMIT;
            if unsafe {
                SetInformationJobObject(
                    job.0,
                    JobObjectExtendedLimitInformation,
                    (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
                    size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
                )
            } == 0
            {
                return Err(last_error("set job limits"));
            }
            let mut cpu = JOBOBJECT_CPU_RATE_CONTROL_INFORMATION::default();
            cpu.ControlFlags =
                JOB_OBJECT_CPU_RATE_CONTROL_ENABLE | JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP;
            cpu.Anonymous.CpuRate = 2_000;
            if unsafe {
                SetInformationJobObject(
                    job.0,
                    JobObjectCpuRateControlInformation,
                    (&cpu as *const JOBOBJECT_CPU_RATE_CONTROL_INFORMATION).cast(),
                    size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>() as u32,
                )
            } == 0
            {
                return Err(last_error("set CPU cap"));
            }
            let completion = OwnedHandle::new(
                unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, null_mut(), 0, 1) },
                "create completion port",
            )?;
            let assoc = JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
                CompletionKey: 1usize as *mut c_void,
                CompletionPort: completion.0,
            };
            if unsafe {
                SetInformationJobObject(
                    job.0,
                    JobObjectAssociateCompletionPortInformation,
                    (&assoc as *const JOBOBJECT_ASSOCIATE_COMPLETION_PORT).cast(),
                    size_of::<JOBOBJECT_ASSOCIATE_COMPLETION_PORT>() as u32,
                )
            } == 0
            {
                return Err(last_error("associate completion port"));
            }
            r.job = Some(job);
            r.completion = Some(completion);
            let listener =
                TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).map_err(|e| e.to_string())?;
            listener.set_nonblocking(true).map_err(|e| e.to_string())?;
            let port = listener.local_addr().map_err(|e| e.to_string())?.port();
            r.listener = Some(listener);
            let mut attrs = AttributeList::new(5)?;
            let caps = SECURITY_CAPABILITIES {
                AppContainerSid: sid,
                Capabilities: null_mut(),
                CapabilityCount: 0,
                Reserved: 0,
            };
            attrs.set(PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, &caps)?;
            let lpac = PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT;
            attrs.set(PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY, &lpac)?;
            let child = PROCESS_CREATION_CHILD_PROCESS_RESTRICTED;
            attrs.set(PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY, &child)?;
            let jobs = [r.job.as_ref().unwrap().0];
            attrs.set(PROC_THREAD_ATTRIBUTE_JOB_LIST, &jobs)?;
            let mitigation = MITIGATION_POLICY;
            attrs.set(PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY, &mitigation)?;
            let app = wide(&staged.to_string_lossy())?;
            let command = format!(
                "\"{}\" --run \"{}\" \"{}\" {}",
                staged.display(),
                sentinel.display(),
                report.display(),
                port
            );
            let mut command = wide(&command)?;
            let current = wide(&profile.to_string_lossy())?;
            let system_root = std::env::var("SystemRoot").map_err(|_| "SystemRoot missing")?;
            let mut environment = Vec::new();
            for item in [
                format!("LOCALAPPDATA={}", profile.display()),
                format!("SystemRoot={system_root}"),
                format!("TEMP={}", profile.display()),
                format!("TMP={}", profile.display()),
            ] {
                environment.extend(item.encode_utf16());
                environment.push(0);
            }
            environment.push(0);
            let mut startup = STARTUPINFOEXW::default();
            startup.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
            startup.lpAttributeList = attrs.pointer();
            let mut pi: PROCESS_INFORMATION = unsafe { zeroed() };
            let flags = CREATE_SUSPENDED
                | CREATE_NO_WINDOW
                | CREATE_UNICODE_ENVIRONMENT
                | EXTENDED_STARTUPINFO_PRESENT
                | if diagnostic {
                    DEBUG_ONLY_THIS_PROCESS
                } else {
                    0
                };
            if unsafe {
                CreateProcessW(
                    app.as_ptr(),
                    command.as_mut_ptr(),
                    null(),
                    null(),
                    0,
                    flags,
                    environment.as_ptr().cast(),
                    current.as_ptr(),
                    &startup.StartupInfo as *const _,
                    &mut pi,
                )
            } == 0
            {
                return Err(last_error("CreateProcessW LPAC"));
            }
            r.process = Some(OwnedHandle::new(pi.hProcess, "process")?);
            r.thread = Some(OwnedHandle::new(pi.hThread, "thread")?);
            if diagnostic {
                r.debug_process_id = Some(pi.dwProcessId);
            }
            let lpac_verification = verify_suspended(
                r.process.as_ref().unwrap().0,
                r.job.as_ref().unwrap().0,
                sid,
            )?;
            if let Some(trace) = &mut debug_trace {
                trace.lpac_verification = lpac_verification.clone();
            }
            if sha256(&fs::read(&staged).map_err(|e| e.to_string())?) != sha256(&canary_bytes)
                || sha256(&fs::read(&sentinel).map_err(|e| e.to_string())?)
                    != sha256(sentinel_bytes)
            {
                return Err("pre-resume immutable hash mismatch".into());
            }
            if unsafe { ResumeThread(r.thread.as_ref().unwrap().0) } == u32::MAX {
                return Err(last_error("resume canary"));
            }
            let mut exit = if let Some(trace) = &mut debug_trace {
                let exit = drive_debug_events(trace)?;
                r.debug_process_id = None;
                exit
            } else {
                let wait =
                    unsafe { WaitForSingleObject(r.process.as_ref().unwrap().0, WALL_TIMEOUT_MS) };
                if wait == WAIT_TIMEOUT {
                    return Err("canary wall timeout".into());
                }
                if wait != WAIT_OBJECT_0 {
                    return Err(format!("unexpected process wait {wait}"));
                }
                0
            };
            if unsafe { GetExitCodeProcess(r.process.as_ref().unwrap().0, &mut exit) } == 0 {
                return Err(last_error("get canary exit"));
            }
            if exit != FIXED_EXIT {
                return Err(format_post_resume_exit(exit, &lpac_verification));
            }
            if r.listener.as_ref().unwrap().accept().is_ok() {
                return Err("loopback connection was accepted".into());
            }
            wait_job_active(r.job.as_ref().unwrap().0, 0)?;
            verify_job_trace(r.completion.as_ref().unwrap().0)?;
            Ok(PassEvidence {
                sid: sid_text,
                binary_hash,
                source_hash,
                contract_hash,
                sentinel_hash,
                repository_hash,
                report_hash: String::new(),
                exit_code: exit,
                lpac_verification,
            })
        })();
        let mut cleanup = r.stop_and_revoke();
        let mut outcome = outcome;
        if cleanup.is_empty() {
            if outcome.is_ok() {
                let profile = r.profile_folder.as_ref().unwrap();
                let report_name = format!("f:forge-denial-report-{}.json", run_id);
                match profile_entries(profile) {
                    Ok(after) => {
                        let mut expected = r.profile_baseline.clone().unwrap_or_default();
                        expected.insert(report_name);
                        if after != expected {
                            outcome = Err(format!("unexpected profile output set: {after:?}"));
                        }
                    }
                    Err(e) => outcome = Err(e),
                }
                if let Ok(pass) = &mut outcome {
                    let report = profile.join(format!("forge-denial-report-{run_id}.json"));
                    match fs::read(&report) {
                        Ok(bytes) => {
                            if let Err(e) = parse_report(&bytes) {
                                outcome = Err(e);
                            } else {
                                pass.report_hash = hex(&sha256(&bytes));
                            }
                        }
                        Err(e) => outcome = Err(format!("read report: {e}")),
                    }
                }
                if let Ok(pass) = &outcome {
                    if hex(&sha256(
                        &fs::read(
                            r.sentinel_dir
                                .as_ref()
                                .unwrap()
                                .join("synthetic-denied-sentinel.bin"),
                        )
                        .unwrap_or_default(),
                    )) != pass.sentinel_hash
                    {
                        outcome = Err("sentinel changed".into());
                    } else {
                        match bounded_inventory(repo) {
                            Ok(after) if inventory_digest(&after) == pass.repository_hash => {}
                            Ok(_) => {
                                outcome = Err("repository inventory changed during trial".into())
                            }
                            Err(e) => outcome = Err(e),
                        }
                    }
                }
            }
        }
        cleanup.extend(r.final_cleanup());
        (run_id, moniker, outcome, cleanup, debug_trace)
    }

    fn json_escape(value: &str) -> String {
        value
            .chars()
            .flat_map(|c| match c {
                '"' => "\\\"".chars().collect::<Vec<_>>(),
                '\\' => "\\\\".chars().collect(),
                c if c.is_control() => format!("\\u{:04x}", c as u32).chars().collect(),
                c => vec![c],
            })
            .collect()
    }

    pub fn main() -> i32 {
        let args: Vec<String> = std::env::args().collect();
        let normal =
            args.len() == 9 && args[1] == "run-once" && args[2] == "--owner-authorized-single-run";
        let standing_diagnostic = args.len() == 9
            && args[1] == "diagnose-once"
            && args[2] == "--standing-delegated-routine-test";
        let repaired_validation = args.len() == 9
            && args[1] == "diagnose-once"
            && args[2] == "--owner-authorized-repaired-validation";
        let diagnostic = standing_diagnostic || repaired_validation;
        if args.len() != 9
            || (!normal && !diagnostic)
            || args[3] != "--canary"
            || args[5] != "--repo-root"
            || args[7] != "--receipt"
        {
            eprintln!("exact bounded runner command required");
            return 2;
        }
        let canary = PathBuf::from(&args[4]);
        let repo = PathBuf::from(&args[6]);
        let receipt = PathBuf::from(&args[8]);
        let repo = match fs::canonicalize(repo) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("repo path: {e}");
                return 2;
            }
        };
        let canary = match fs::canonicalize(canary) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("canary path: {e}");
                return 2;
            }
        };
        if diagnostic {
            let actual = match fs::read(&canary) {
                Ok(bytes) => hex(&sha256(&bytes)),
                Err(e) => {
                    eprintln!("read diagnostic canary: {e}");
                    return 2;
                }
            };
            if actual != "25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856" {
                eprintln!("diagnostic mode requires the exact retained static candidate");
                return 2;
            }
        }
        let parent = match receipt.parent().and_then(|p| fs::canonicalize(p).ok()) {
            Some(p) => p,
            None => {
                eprintln!("receipt parent missing");
                return 2;
            }
        };
        if !parent.starts_with(repo.join("evidence"))
            || receipt.extension().and_then(|e| e.to_str()) != Some("json")
            || receipt.exists()
            || (standing_diagnostic
                && receipt.file_name().and_then(|name| name.to_str())
                    != Some("trial-5-debug-events.json"))
            || (repaired_validation
                && receipt.file_name().and_then(|name| name.to_str())
                    != Some("repaired-observer-validation.json"))
        {
            eprintln!("receipt path is not a fresh JSON file under repository evidence");
            return 2;
        }
        let runner_hash = std::env::current_exe()
            .ok()
            .and_then(|path| fs::read(path).ok())
            .map(|bytes| hex(&sha256(&bytes)))
            .unwrap_or_default();
        let (run_id, moniker, result, cleanup, debug_trace) = trial(&canary, &repo, diagnostic);
        let cleanup_ok = cleanup.is_empty();
        let (status, error, body) = if diagnostic {
            let error = match result {
                Ok(_) => String::new(),
                Err(e) => e,
            };
            let complete = cleanup_ok
                && debug_trace
                    .as_ref()
                    .is_some_and(|trace| trace.exit_code.is_some());
            let body = debug_trace
                .as_ref()
                .map(debug_trace_json)
                .unwrap_or_default();
            (
                if complete {
                    "diagnostic_completed"
                } else {
                    "failed"
                },
                error,
                body,
            )
        } else {
            match result {
                Ok(pass) if cleanup_ok => (
                    "passed",
                    String::new(),
                    format!(
                        "\"sid\":\"{}\",\"binary_sha256\":\"{}\",\"source_sha256\":\"{}\",\"contract_sha256\":\"{}\",\"sentinel_sha256\":\"{}\",\"repository_inventory_sha256\":\"{}\",\"report_sha256\":\"{}\",\"exit_code\":{},\"lpac_verification\":\"{}\"",
                        pass.sid,
                        pass.binary_hash,
                        pass.source_hash,
                        pass.contract_hash,
                        pass.sentinel_hash,
                        pass.repository_hash,
                        pass.report_hash,
                        pass.exit_code,
                        pass.lpac_verification
                    ),
                ),
                Ok(_) => ("failed", "cleanup failure".into(), String::new()),
                Err(e) => ("failed", e, String::new()),
            }
        };
        let cleanup_json = cleanup
            .iter()
            .map(|e| format!("\"{}\"", json_escape(e)))
            .collect::<Vec<_>>()
            .join(",");
        let separator = if body.is_empty() { "" } else { "," };
        let bytes = format!(
            "{{\"schema\":1,\"trial\":\"{}\",\"run_id\":\"{}\",\"moniker\":\"{}\",\"status\":\"{status}\",\"contract_version\":{CONTRACT_VERSION},\"runner_sha256\":\"{}\",\"cleanup_ok\":{cleanup_ok},\"error\":\"{}\",\"cleanup_errors\":[{}]{}{}}}",
            if diagnostic {
                "P7b-1b-diagnostic"
            } else {
                "P7b-1b"
            },
            json_escape(&run_id),
            json_escape(&moniker),
            runner_hash,
            json_escape(&error),
            cleanup_json,
            separator,
            body
        );
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&receipt)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("receipt create: {e}");
                return 3;
            }
        };
        if let Err(e) = file
            .write_all(bytes.as_bytes())
            .and_then(|_| file.sync_all())
        {
            eprintln!("receipt write: {e}");
            return 3;
        }
        println!("{bytes}");
        if status == "passed" || status == "diagnostic_completed" {
            0
        } else {
            1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn diagnostic_handles_only_the_bootstrap_breakpoint() {
            assert_eq!(exception_continue_status(0x8000_0003), DBG_CONTINUE);
            for code in [0xC000_0005, 0xC000_0142, 0x8000_0004] {
                assert_eq!(exception_continue_status(code), DBG_EXCEPTION_NOT_HANDLED);
            }
        }

        #[test]
        fn diagnostic_receipt_is_trace_bounded_and_authority_negative() {
            let trace = DebugTrace {
                candidate_sha256: "a".repeat(64),
                lpac_verification: "access-check-after-class46-error87".into(),
                events: vec![DebugEventEvidence {
                    sequence: 0,
                    kind: "exit_process",
                    exit_code: Some(0xC000_0142),
                    ..Default::default()
                }],
                exit_code: Some(0xC000_0142),
            };
            let json = debug_trace_json(&trace);
            assert!(json.contains("\"event_count\":1"));
            assert!(json.contains("\"denial_proved\":false"));
            assert!(json.contains("\"runtime_cause_proved\":false"));
            assert!(json.contains("\"capability_added\":false"));
            assert!(!json.contains("memory"));
            assert!(!json.contains("stack"));
        }
    }
}

#[cfg(windows)]
fn main() {
    std::process::exit(windows_runner::main());
}

#[cfg(not(windows))]
fn main() {
    eprintln!("containment canary runner is Windows-only");
    std::process::exit(75);
}
