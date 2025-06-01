# `pini`

Basic INI file formatter.

## Tools used for development

- Fedora Linux 42
- Rust 1.86.0

## Supported options

### `AssignmentSpaces`

- `None` -> `key=value`.
- `Both` -> `key = value`.
- `LeftOnly` -> `key =value`.
- `RightOnly` -> `key= value`.

### `EmptyLinesBeforeSection`

E.g., for `1`:

```text
key=value
[section_name]
```

becomes:

```text
key=value

[section_name]
```

### `EmptyLineAtTheEnd`

E.g., for `true`:

```text
last_line = value
```

becomes:

```text
last_line = value

```
