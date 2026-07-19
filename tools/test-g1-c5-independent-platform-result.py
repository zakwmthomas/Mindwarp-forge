#!/usr/bin/env python3
"""Hostile fixtures for replayable retained C5 attestation evidence."""
from __future__ import annotations

import base64
import importlib.util
import json
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


fixtures_module = load("c5_external_fixtures", ROOT / "tools/test-g1-c5-external-receipt.py")
importer = fixtures_module.v
retained_module = load(
    "c5_retained_verifier", ROOT / "tools/verify-g1-c5-independent-platform-result.py"
)


class Completed:
    returncode = 0
    stdout = '[{"verification":"fixture"}]\n'
    stderr = ""


last_command = []
expected_result = b""
expected_bundle = b""


def runner(command, **_kwargs):
    last_command[:] = command
    if Path(command[3]).read_bytes() != expected_result:
        raise AssertionError("replay did not receive exact retained result bytes")
    bundle_index = command.index("--bundle") + 1
    if Path(command[bundle_index]).read_bytes() != expected_bundle:
        raise AssertionError("replay did not receive exact retained bundle bytes")
    source = "a" * 40
    expected = [
        "gh-fixture", "attestation", "verify", command[3], "--repo", "x/y",
        "--bundle", command[bundle_index], "--deny-self-hosted-runners",
        "--source-digest", source, "--signer-workflow",
        "x/y/.github/workflows/g1-c5-independent-platform.yml",
        "--signer-digest", source, "--format", "json",
    ]
    if command != expected or "--custom-trusted-root" in command:
        raise AssertionError("retained replay command changed")
    return Completed()


def reseal(receipt):
    receipt.pop("receipt_sha256", None)
    receipt["receipt_sha256"] = importer.digest(receipt)


def build():
    request, result = fixtures_module.fixtures()
    request_raw = (json.dumps(request, sort_keys=True, indent=2) + "\n").encode()
    result_raw = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    bundle = b'{"fixture":"bundle"}\n'
    receipt = importer.build_retained_receipt(request, result, request_raw, result_raw, bundle)
    local = {
        "source_commit": request["source"]["source_commit"],
        "tracked_tree_manifest_sha256": request["source"]["tracked_tree_manifest_sha256"],
        "bounded_source_manifest_sha256": request["source"]["bounded_source_manifest_sha256"],
        "semantic_receipt_sha256": request["semantic"]["expected_sha256"],
        "independent_second_platform_execution": False,
        "promotion_authority": False,
        "c6_authority": False,
    }
    return receipt, local


def verify(receipt, local):
    global expected_result, expected_bundle
    expected_result = base64.b64decode(receipt["result_json_base64"])
    expected_bundle = base64.b64decode(receipt["attestation_bundle_base64"])
    return retained_module.verify_retained(
        receipt,
        local=local,
        runner=runner,
        gh_path="gh-fixture",
        source_checker=lambda *_: None,
        verify_git=False,
    )


def reject(name, mutation, reseal_after=True):
    receipt, local = build()
    mutation(receipt)
    if reseal_after:
        reseal(receipt)
    try:
        verify(receipt, local)
    except Exception:
        return
    raise AssertionError(f"retained hostile admitted: {name}")


def main():
    receipt, local = build()
    verify(receipt, local)
    if not last_command:
        raise AssertionError("retained replay command was not captured")
    cases = {
        "unknown-field": lambda r: r.update(extra=True),
        "schema-boolean": lambda r: r.update(schema_version=True),
        "hosted-runner-integer": lambda r: r.update(hosted_runner=1),
        "promotion-authority-integer": lambda r: r["authority"].update(promotion_authority=0),
        "promotion-authority": lambda r: r["authority"].update(promotion_authority=True),
        "c6-authority": lambda r: r["authority"].update(activate_c6=True),
        "repository-mutation": lambda r: r["authority"].update(repository_mutation=True),
        "self-hosted": lambda r: r.update(hosted_runner=False),
        "same-platform": lambda r: r["platform"].update(
            os="windows", target="x86_64-pc-windows-msvc"
        ),
        "source-commit": lambda r: r.update(source_commit="d" * 40),
        "semantic-receipt": lambda r: r.update(semantic_receipt_sha256="d" * 64),
        "bundle-hash": lambda r: r.update(attestation_bundle_sha256="d" * 64),
        "result-bytes": lambda r: r.update(result_json_base64=r["request_json_base64"]),
        "invalid-base64": lambda r: r.update(attestation_bundle_base64="!not-base64!"),
        "status": lambda r: r.update(status="candidate"),
    }
    for name, mutation in cases.items():
        reject(name, mutation)
    receipt, local = build()
    local["c6_authority"] = True
    try:
        verify(receipt, local)
        raise AssertionError("local C6 authority admitted")
    except ValueError:
        pass
    reject("receipt-self-hash", lambda r: r.update(status="candidate"), reseal_after=False)
    receipt, local = build()

    def failed_runner(*_args, **_kwargs):
        class Failed:
            returncode = 1
            stdout = ""
            stderr = "bad signature"

        return Failed()

    try:
        retained_module.verify_retained(
            receipt, local=local, runner=failed_runner, gh_path="gh-fixture", verify_git=False
        )
        raise AssertionError("failed gh replay admitted")
    except ValueError:
        pass
    with tempfile.TemporaryDirectory() as td:
        duplicate = Path(td) / "duplicate.json"
        duplicate.write_text('{"a":1,"a":2}', encoding="utf-8")
        try:
            retained_module.load_retained(duplicate)
            raise AssertionError("duplicate retained key admitted")
        except ValueError:
            pass
    print(
        f"C5 retained independent-platform verifier passed: valid exact-byte replay plus {len(cases) + 4} hostile cases."
    )


if __name__ == "__main__":
    main()
