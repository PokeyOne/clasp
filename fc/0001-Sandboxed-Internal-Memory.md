# FC 0001 - Sandboxed Internal Memory

Essentially when a program is run it would have a sandbox memory it could work
with and create, read, and write files. They could potentially be temporary
files that delete at the end of a session, or they could be persistent files
that persist in between runs of the program on one computer. They should not
be persisted if the executable is moved to a different file, so they must be
store on the computer somewhere.

## Benefits
1. list some benefits here
1. Essentially write reasons why this should be implemented

## Downsides
1. List some of the things that this would change for the worse
1. These can be any small thing

## Backwards Compatibility
no

## Attached PRs
none

## Attached Issues
none

## Long descriptions
This is where the small details should be managed