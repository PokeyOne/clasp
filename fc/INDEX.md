# Feature Consideration Index
[template](template.md)

The following FC's exist:

## 0001 - Sandboxed Internal Memory
[full-description](0001-Sandboxed-Internal-Memory.md)

Essentially when a program is run it would have a sandbox memory it could work
with and create, read, and write files. They could potentially be temporary
files that delete at the end of a session, or they could be persistent files
that persist in between runs of the program on one computer. They should not
be persisted if the executable is moved to a different file, so they must be
store on the computer somewhere.