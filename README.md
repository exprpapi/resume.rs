# `resume.rs`

> It's not about the resume of the man, but about the man in the resume.

A standalone templating resume generator written in Rust.

| Input `yaml`                | Output `pdf`                 |
| --------------------------- | ---------------------------- |
| ![input](./demo_input.png)  | ![output](./demo_output.png) |

Looks for a [`resume.yaml`](./resume.yaml) file and generates both [`resume.tex`](./resume.tex) and [`resume.pdf`](./resume.pdf).

The generated [`resume.tex`](./resume.tex) is human-readable and can serve for further customizations.

## Usage

To generate `.tex` and `.pdf` from `resume.yaml`
```bash
./resume.rs build resume.yaml
```

or, equivalently,

```bash
./resume.rs build
```

To watch and automatically rebuild `.tex` and `.pdf` on 'resume.yaml' file save
```bash
./resume.rs watch resume.yaml
```

or, equivalently

```bash
./resume.rs watch
```


## Installation

Since this program uses the experimental `cargo-script` feature, only making it executable is required:
```bash
chmod +x resume.rs
```

## Dependencies

Only uses `cargo`. You might need to install the nightly toolchain.

## Notes

The `yaml` schema is by all means not yet ratified.

## Demo

![demo](./demo.gif)
