# mdfind

Lokales CLI-Tool zur schnellen Suche in Markdown-Notizen.

Ziel: eigener Ersatz für tldr / cheat.sh auf Basis der eigenen Wissensbasis.

---

## Features

- durchsucht `.md` Dateien rekursiv in `~/docs/mdfind`
- erkennt Überschriften (`#`, `##`, …)
- Kategorie = Ordnerstruktur
- Tags aus Datei (`tags:`)
- Filter nach Tags
- fzf-kompatible Ausgabe
- interaktive Erstellung neuer Notizen

---

## Installation
Unter Umständen mit nightly Rust-Toolchain:

```bash
cargo +nightly build --release
````

Binary liegt danach in:

```text
target/release/mdfind
```

Optional:

```bash
cp mdfind/target/release/mdfind ~/.local/bin/mdfind
```

---

## Verzeichnisstruktur

```text
~/docs/mdfind/
├── linux/
├── python/
├── rust/
```

→ Ordner = Kategorie (wird automatisch als Tag verwendet)

---

## Dateiformat

```markdown
# Titel

date: 2026-03-18
tags: linux,fs

## Abschnitt
Inhalt
```

---

## Nutzung

### Suche

```bash
mdfind
mdfind inode
```

---

### Tag-Filter

```bash
mdfind -t linux
mdfind inode -t fs
```

* berücksichtigt:

  * Ordnername (Kategorie)
  * `tags:` in Datei

---

### Output-Format

```text
path:line: title
```

Beispiel:

```text
linux/fs.md:42: Inodes
```

---

## fzf Integration

### Interaktiv suchen + öffnen

```bash
mdfind | fzf \
  --delimiter ':' \
  --preview 'bat {1} --highlight-line {2}' \
  --bind 'enter:execute(nvim +{2} {1})'
```

---

### als Funktion

```bash
mde() {
  mdfind | fzf \
    --delimiter ':' \
    --preview 'bat {1} --highlight-line {2}' \
    --bind 'enter:execute(nvim +{2} {1})'
}
```

---

## Notizen erstellen

```bash
mdfind create
```

Interaktiv:

* Title
* Tags
* Category (Ordner)

Erzeugt:

```markdown
# Titel

date: YYYY-MM-DD
tags: ...
```

Pfad:

```text
~/docs/mdfind/<category>/<title>.md
```

---

## Design

* kein Markdown-Parser (Regex reicht)
* einfache Tags (kein YAML)
* Kategorie über Ordner
* stdout-first (Unix-Philosophie)

---

## Erweiterungen (geplant)

* Cache / Index (mtime-basiert)
* Ranking nach Nutzung
* Volltextsuche gewichten
* SQLite Backend
* Neovim Integration (Quickfix)

---

## Abhängigkeiten

* walkdir
* regex
* clap
* chrono
* dialoguer

---

## Minimal Workflow

```bash
mdfind "problem" | fzf
```

→ auswählen → direkt in Editor

---

## Motivation

* schneller als Websuche
* eigenes Wissen zentralisiert
* reproduzierbare Lösungen

