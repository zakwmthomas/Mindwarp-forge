use std::{
    env, fs,
    path::PathBuf,
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

use forge_kernel::persistence::PersistentForge;
use serde::de::DeserializeOwned;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("forge-federate: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let command = args.first().ok_or_else(usage)?;
    if matches!(
        command.as_str(),
        "claim-workstream-writer" | "assert-workstream-writer" | "release-workstream-writer"
    ) {
        return run_writer_lease(command, &args[1..]);
    }
    if args.len() != 3 {
        return Err(usage());
    }
    let database = PathBuf::from(&args[1]);
    let record_path = PathBuf::from(&args[2]);
    let forge = PersistentForge::open(database).map_err(debug)?;
    match command.as_str() {
        "backfill-knowledge-disposable" => {
            if record_path != PathBuf::from("--disposable-fixture") {
                return Err(usage());
            }
            forge.backfill_knowledge_records().map_err(debug)?;
        }
        "finalize-knowledge-generation-disposable" => {
            if record_path != PathBuf::from("--disposable-fixture") {
                return Err(usage());
            }
            forge
                .finalize_existing_knowledge_generation()
                .map_err(debug)?;
        }
        "register-project" => forge
            .record_project(&read_record(&record_path)?)
            .map_err(debug)?,
        "register-workstream" => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(debug)?
                .as_millis() as u64;
            forge
                .record_workstream(&read_record(&record_path)?, now)
                .map_err(debug)?;
        }
        "route-session" => forge
            .record_session_route(&read_record(&record_path)?)
            .map_err(debug)?,
        "link-projects" => forge
            .record_cross_project_link(&read_record(&record_path)?)
            .map_err(debug)?,
        _ => return Err(usage()),
    }
    println!("recorded {command} from {}", record_path.display());
    Ok(())
}

fn run_writer_lease(command: &str, args: &[String]) -> Result<(), String> {
    let expected = match command {
        "claim-workstream-writer" => 5,
        "assert-workstream-writer" => 4,
        "release-workstream-writer" => 3,
        _ => return Err(usage()),
    };
    if args.len() != expected {
        return Err(usage());
    }
    let database = PathBuf::from(&args[0]);
    let workstream_id = &args[1];
    let session_id = &args[2];
    let forge = PersistentForge::open(database).map_err(debug)?;
    let now = now_unix_ms()?;
    match command {
        "claim-workstream-writer" => {
            let checkpoint_sha256 = &args[3];
            let ttl_seconds = args[4].parse::<u64>().map_err(debug)?;
            let ttl_ms = ttl_seconds
                .checked_mul(1_000)
                .ok_or_else(|| "writer lease TTL overflow".to_string())?;
            let record = forge
                .claim_writer_lease(workstream_id, session_id, checkpoint_sha256, now, ttl_ms)
                .map_err(debug)?;
            println!("{}", serde_json::to_string(&record).map_err(debug)?);
        }
        "assert-workstream-writer" => {
            forge
                .assert_writer_lease(workstream_id, session_id, &args[3], now)
                .map_err(debug)?;
            println!("writer lease valid for {session_id} on {workstream_id}");
        }
        "release-workstream-writer" => {
            let record = forge
                .release_writer_lease(workstream_id, session_id, now)
                .map_err(debug)?;
            println!("{}", serde_json::to_string(&record).map_err(debug)?);
        }
        _ => return Err(usage()),
    }
    Ok(())
}

fn now_unix_ms() -> Result<u64, String> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(debug)?
        .as_millis() as u64)
}

fn read_record<T: DeserializeOwned>(path: &PathBuf) -> Result<T, String> {
    serde_json::from_slice(&fs::read(path).map_err(debug)?).map_err(debug)
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

fn usage() -> String {
    "usage: forge-federate <register-project|register-workstream|route-session|link-projects> <forge.sqlite3> <record.json> OR forge-federate <backfill-knowledge-disposable|finalize-knowledge-generation-disposable> <fixture.sqlite3> --disposable-fixture OR forge-federate <claim-workstream-writer|assert-workstream-writer> <forge.sqlite3> <workstream-id> <session-id> <checkpoint-sha256> [ttl-seconds] OR forge-federate release-workstream-writer <forge.sqlite3> <workstream-id> <session-id>".into()
}
