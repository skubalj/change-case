`change-case`: Command Line Case Conversions
=============

Consume newline delimited identifers from stdin and convert each to a specified
case. This program is intended to be used when refactoring field or method names.

Say that you are writing an application in a language that idiomatically uses 
snake case identifiers for fields, but you are copying field names from a JSON
schema that uses lower camel case. (eg: writing a Rust app to match a
pre-existing schema) You could update all the field names by hand, but that's 
time consuming busywork. Alternatively, you could try to generate your interface
from the schema, but sometimes you don't want the overhead of setting up a code
generator.

This program bridges the gap, providing a lightweight and composable tool for
case conversions.

## Examples

To make it easy to integrate into shell scripts and other pipelines, this
program reads from stdin and writes to stdout. If you are working in an IDE,
you can copy your identifiers to the system clipboard, then use `xclip` (linux)
or `pbcopy`/`pbpaste` (mac) to feed that input into the program.

```bash
xclip -o | change-case snake | xclip -i
```

Alternatively, you can run the program interactively. Type or paste identifiers
into the terminal, and the converted form will be written to stdout. Press 
`Ctrl+D` to close stdin and exit the program. This example redirects the output
to a temporary file.

```bash
change-case lower-camel > /tmp/out.txt
```

## Acknowledgements

`change-case` is made possible thanks to the generous contributions of others!

| Crate      | Owner / Maintainer                        | License           |
| ---------- | ----------------------------------------- | ----------------- |
| anyhow     | David Tolnay                              | MIT or Apache-2.0 |
| clap       | Kevin K. / clap-rs Admins                 | MIT or Apache-2.0 |

## License

Copyright (C) 2025 Joseph Skubal

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
