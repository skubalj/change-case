`change-case`
=============

Consume newline delimited identifers from stdin and convert each to a specified
case. This program is intended to be used when refactoring field or method names.

## Examples

Read identifiers from the system clipboard, change each to snake case, then 
store the data back in the system clipboard

```bash
xclip -o | change-case snake | xclip -i
```

Read identifiers from stdin, convert to camel case, and write the output to a
temp file
```bash
change-case lower-camel > /tmp/out.txt
```
