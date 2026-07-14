//! One-shot synthetic denial probe. This binary is intentionally AppContainer-only.

#[cfg(windows)]
use std::{
    env, fs,
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
    path::PathBuf,
    process::Command,
    time::Duration,
};

#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{GetTokenInformation, TOKEN_QUERY, TokenCapabilities, TokenIsAppContainer},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

#[cfg(windows)]
const FIXED_EXIT: i32 = 73;

#[cfg(windows)]
fn token_u32(class: i32) -> Result<u32, i32> {
    let mut token: HANDLE = std::ptr::null_mut();
    // SAFETY: the current process pseudo-handle is valid; token is closed below.
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
        return Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(-1));
    }
    let mut value = 0u32;
    let mut returned = 0u32;
    // SAFETY: value is a writable u32 buffer appropriate for TokenIsAppContainer
    // and the leading GroupCount field of TokenCapabilities.
    let ok = unsafe {
        GetTokenInformation(
            token,
            class,
            (&mut value as *mut u32).cast(),
            size_of::<u32>() as u32,
            &mut returned,
        )
    };
    // SAFETY: token came from OpenProcessToken.
    unsafe { CloseHandle(token) };
    if ok == 0 {
        Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(-1))
    } else {
        Ok(value)
    }
}

#[cfg(windows)]
fn denied_error<T>(result: std::io::Result<T>) -> i32 {
    match result {
        Ok(_) => 0,
        Err(error) => error.raw_os_error().unwrap_or(-1),
    }
}

#[cfg(windows)]
fn run() -> Result<(), String> {
    let args: Vec<_> = env::args_os().collect();
    if args.get(1).is_some_and(|arg| arg == "--child") {
        return Err("restricted child unexpectedly started".into());
    }
    if args.len() != 5 || args[1] != "--run" {
        return Err("invalid fixed canary command".into());
    }
    let sentinel = PathBuf::from(&args[2]);
    let report = PathBuf::from(&args[3]);
    let port = args[4]
        .to_string_lossy()
        .parse::<u16>()
        .map_err(|_| "invalid loopback port")?;

    let token_is_appcontainer = token_u32(TokenIsAppContainer).map_err(|e| e.to_string())?;
    let capability_count = token_u32(TokenCapabilities).map_err(|e| e.to_string())?;
    let sentinel_error = denied_error(fs::read(&sentinel));
    let self_path = env::current_exe().map_err(|error| error.to_string())?;
    let child_error = denied_error(Command::new(self_path).arg("--child").spawn());
    let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let loopback_error = denied_error(TcpStream::connect_timeout(
        &address.into(),
        Duration::from_millis(750),
    ));
    let bytes = format!(
        "{{\"schema\":1,\"token_is_appcontainer\":{},\"capability_count\":{},\"sentinel_error\":{},\"child_error\":{},\"loopback_error\":{}}}",
        token_is_appcontainer, capability_count, sentinel_error, child_error, loopback_error
    );
    if bytes.len() > 256 {
        return Err("report exceeded fixed bound".into());
    }
    fs::write(report, bytes.as_bytes()).map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(windows)]
fn main() {
    match run() {
        Ok(()) => std::process::exit(FIXED_EXIT),
        Err(_) => std::process::exit(74),
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("containment denial canary is Windows-only");
    std::process::exit(75);
}
