# Codex Local Capture Contract v0.1

The local Codex adapter is a read-only evidence source. It may read only
locally persisted Codex Desktop JSONL sessions and only accepts visible
`user/input_text` and `assistant/output_text` records.

Each accepted record has a deterministic session-and-record-hash identity and
a durable cursor. Replays are idempotent. Assistant records may create
proposed candidates. Captured user records are explicitly imported evidence,
not direct authorization, even where their visible speaker is the project
owner.

The adapter never approves, promotes, rejects, executes, applies, or rolls
back code. It never reads screen pixels, OCR, clipboard contents, browser DOM,
network traffic, system/developer/tool records, or unknown content types.

Unexpected JSON, invalid metadata, non-UTF-8 content, or a source identity
change pauses the affected source and records an error. It must not infer or
transform a replacement format. Resuming retries the same source; it does not
enable a weaker fallback.
