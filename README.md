# `resume.rs`

> It's not about the resume of the man, but about the man in the resume.

A standalone glorified templating resume generator written in Rust.

Looks for a `resume.yaml` file and generates both `resume.tex` and `resume.pdf`

The `yaml` schema is by all means not yet ratified.

## Usage

```bash
chmod +x resume.rs
./resume.rs
```

## Dependencies

Only uses `cargo`. You might need to install the nightly toolchain.
