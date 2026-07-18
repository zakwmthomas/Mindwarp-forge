use std::{env, path::PathBuf, process::ExitCode};

use forge_kernel::persistence::{KnowledgeSearchQuery, PersistentForge};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("forge-query: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let text = args.next().ok_or_else(usage)?;
    let mut database = None;
    let mut project_id = None;
    let mut facet_type = None;
    let mut workstream_id = None;
    let mut entity_id = None;
    let mut system_id = None;
    let mut source_actor = None;
    let mut lifecycle_state = None;
    let mut limit = 20_usize;
    while let Some(flag) = args.next() {
        let value = args.next().ok_or_else(usage)?;
        match flag.as_str() {
            "--database" => database = Some(PathBuf::from(value)),
            "--project" => project_id = Some(value),
            "--type" => facet_type = Some(value),
            "--workstream" => workstream_id = Some(value),
            "--entity" => entity_id = Some(value),
            "--system" => system_id = Some(value),
            "--actor" => source_actor = Some(value),
            "--state" => lifecycle_state = Some(value),
            "--limit" => limit = value.parse().map_err(|_| "--limit must be an integer")?,
            _ => return Err(usage()),
        }
    }
    let database = database.ok_or_else(|| "--database is required".to_owned())?;
    let hits = PersistentForge::search_database(
        database,
        &KnowledgeSearchQuery {
            text,
            facet_type,
            project_id,
            workstream_id,
            entity_id,
            system_id,
            source_actor,
            lifecycle_state,
            limit,
        },
    )
    .map_err(|error| format!("query failed: {error:?}"))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&hits).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn usage() -> String {
    "usage: forge-query <fts-query> --database <forge.sqlite3> [--project ID] [--workstream ID] [--entity ID] [--system ID] [--type FACET] [--actor NAME] [--state STATE] [--limit 1..500]".into()
}
