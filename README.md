# `pini`

Basic INI file formatter.

## Tools used for development

- Fedora Linux 42
- Rust 1.86.0

## Build and install

Build from source:

```bash
cargo build --release
```

Install from source:

```bash
cargo install --path .
```

### Create packages

Packages are created with additional software:

```bash
cargo install cargo-deb cargo-generate-rpm cargo-aur
```

Application must be built and stripped:

```bash
cargo build --release
strip -s target/release/pini
```

Create packages:

```bash
cargo deb
cargo generate-rpm
cargo aur
```

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
