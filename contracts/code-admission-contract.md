# Code Admission Contract v0.1

Code admission is an evidence boundary, not a code-writing system. The first
Forge capability accepts explicitly pasted source text plus a source ID and a
safe repository-relative target path.

It must:

1. preserve raw code bytes as immutable evidence;
2. create a separate canonical manifest object containing source ID, path,
   language, and raw-code object ID;
3. create a candidate from that manifest;
4. reject empty IDs, empty code, oversized input, absolute paths, path
   traversal, backslashes, and blank path segments;
5. provide an idempotent receipt for the same source/path/language/code;
6. never write source files, execute code, alter the working tree, grant
   approval, or promote a candidate.

Applying an approved code candidate to a repository is a separate,
owner-authorized module with its own preview, diff, verification, and rollback
requirements.
