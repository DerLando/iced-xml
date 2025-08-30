# iced-xml

An *unofficial* xml templating language for building [Iced]() user interfaces.

## TODO

- [x] Move parsing into it's own crate
  - [x] Think about having the parsed types in core. Maybe the parsing itself happens there then, too
- [x] Move the previewing app into it's own crate
  - [x] Allow to run the app with a file path to preview
  - [ ] Add a file picker to change the preview file at runtime
- [ ] Add crate with derive macro, that generates a IcedComponent impl from a ParsedComponent/ParsedWindow input
