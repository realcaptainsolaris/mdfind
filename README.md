# mdfind

Simple CLI tool to create and search Markdown notes.

Designed to be used together with `fzf` for fast, keyboard-driven navigation.

---

## Features

- Create notes with title, tags, and category
- Search Markdown headers (`#`, `##`, ...)
- Filter by tag or folder
- Works great with `fzf` + `nvim`

---

## Installation

### 1. Clone & build

Requires Rust.

```bash
git clone https://github.com/realcaptainsolaris/mdfind
cd mdfind
cargo +nightly build --release
````

Binary will be here:

```bash
target/release/mdfind
```

You may want to move it:

```bash
cp target/release/mdfind ~/.local/bin/
```

---

## Notes Directory

By default, notes are stored in:

```bash
~/docs/mdfind
```

You can change this in the code:

```rust
const DIR_NAME: &str = "docs/mdfind";
```

---

## Usage

### Create a note

```bash
mdfind create
```

You will be prompted for:

* title
* tags
* category (folder)

---

### Search notes

```bash
mdfind <query>
mdfind --tag rust
mdfind <query> --tag rust
```

Search is performed on Markdown headers.

---

## fzf Integration

This tool is intended to be used with `fzf`.

### Requirements

* `fzf`
* `bat` (on Debian/Ubuntu this is `batcat`)
* `nvim`

---

### Zsh

Add to your `.zshrc`:

```bash
mde() {
  mdfind | fzf --ansi \
    --preview 'batcat $(echo {} | cut -d: -f1) --highlight-line $(echo {} | cut -d: -f2)' \
    --bind 'enter:execute(nvim +$(echo {} | cut -d: -f2) $(echo {} | cut -d: -f1))'
}
```

---

### Bash

Add to your `.bashrc`:

```bash
mde() {
  mdfind | fzf --ansi \
    --preview 'batcat $(echo {} | cut -d: -f1) --highlight-line $(echo {} | cut -d: -f2)' \
    --bind 'enter:execute(nvim +$(echo {} | cut -d: -f2) $(echo {} | cut -d: -f1))'
}
```

---

## Output Format

Each result line:

```
path:line: title
```

This is parsed by `fzf` using `cut`.

---

## Notes

* `batcat` is required for preview (Debian/Ubuntu naming)
* Each file returns at most one match
* Only headers are searched (not full content)

---

## Philosophy

* minimal
* fast
* terminal-first workflow
* no database, no indexing

---

## License

MIT

```

---

wenn du willst, kann ich dir noch:

- ein gif für github bauen (wirkt massiv besser)
- oder einen „pro“ fzf workflow mit preview scrolling etc.
```
