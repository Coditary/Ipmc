# Impact Map CLI (`ipmc`)

**Impact Maps als Code — versionierbar, diffbar, renderbar.**

`ipmc` ist ein CLI-Tool, das [Impact Maps](https://www.impactmapping.org/) aus einer strukturierten [KDL](https://kdl.dev/)-Datei in hochwertige, farbcodierte Graphen übersetzt. Statt Mindmaps in Whiteboard-Tools zu pflegen, beschreibst du Ziele, Metriken, Akteure, Impacts und Deliverables als Code — und generierst daraus SVG-, PNG- oder DOT-Dateien.

```
KDL-Datei  →  Parser  →  Graphviz DOT  →  SVG / PNG
```

---

## Warum Impact Maps als Code?

Impact Mapping verbindet **Geschäftsziele** mit **messbaren Metriken**, **Akteuren**, deren **Verhalten (Impacts)** und konkreten **Deliverables**. Klassische Mindmap-Tools sind schwer versionierbar und kollaborativ schwer zu reviewen.

Mit `ipmc` bekommst du:

- **Git-freundlich** — Impact Maps als Textdatei, diffbar und reviewbar
- **Automatisierbar** — CI/CD, Skripte, Batch-Rendering
- **Konsistent** — einheitliches Layout und Farbschema pro Goal
- **Priorisierung sichtbar** — Prio und Story Points direkt in den Knoten

---

## Features

| Feature | Beschreibung |
|---------|--------------|
| KDL-Parser | Deklaratives Format für Goals, Metrics, Actors, Impacts, Deliverables |
| Multi-Goal | Eine Datei, viele Goals — einzeln oder alle auf einmal rendern |
| Formate | SVG (Standard), PNG, reines DOT |
| Styled Output | Farbcodierte HTML-Labels mit Legende, Metriken und Prioritäten |
| Auto-Open | Generierte Map direkt im Standard-Viewer öffnen |
| Cross-Platform | Linux, macOS, Windows — Prebuilts via GitHub Releases |

---

## Voraussetzungen

| Abhängigkeit | Wann nötig | Installation |
|--------------|------------|--------------|
| **Rust** (1.70+) | Build from source | [rustup.rs](https://rustup.rs/) |
| **Graphviz** (`dot`) | SVG/PNG-Export | `sudo dnf install graphviz` (Fedora) · `brew install graphviz` (macOS) · [graphviz.org](https://graphviz.org/download/) |

> Nur `--format dot` braucht kein Graphviz — die `.dot`-Datei wird direkt geschrieben.

---

## Installation

### Prebuilt Binary (empfohlen)

Lade die passende Binary von [GitHub Releases](https://github.com/Coditary/Ipmc/releases) herunter (getriggert durch Tags `v*`):

```bash
# Beispiel Linux x86_64
curl -LO https://github.com/Coditary/Ipmc/releases/latest/download/ipmc-x86_64-unknown-linux-gnu
chmod +x ipmc-x86_64-unknown-linux-gnu
sudo mv ipmc-x86_64-unknown-linux-gnu /usr/local/bin/ipmc
```

Verfügbare Targets: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc`

### Aus dem Quellcode

```bash
git clone https://github.com/Coditary/Ipmc.git
cd Ipmc
cargo build --release
# Binary: target/release/impact-map-cli
```

---

## Schnellstart

```bash
# Alle Goals aus example.kdl als SVG rendern
cargo run --release -- example.kdl

# Ein bestimmtes Goal
cargo run --release -- example.kdl -g GOAL-02

# Als PNG, mit Auto-Open und verbose Logging
cargo run --release -- example.kdl -g GOAL-02 -f png --open -v
```

Ergebnis: `GOAL-02.svg` (bzw. `GOAL-02.png`) im aktuellen Verzeichnis.

---

## CLI-Referenz

```
ipmc <INPUT> [OPTIONS]
```

| Flag | Kurz | Default | Beschreibung |
|------|------|---------|--------------|
| `--goal` | `-g` | — | Nur dieses Goal rendern (z. B. `GOAL-02`) |
| `--out` | `-o` | `.` | Ausgabeverzeichnis **oder** exakter Dateiname (ohne Endung) bei `-g` |
| `--format` | `-f` | `svg` | `svg` · `png` · `dot` |
| `--keep-dot` | — | `false` | Zwischendatei `.dot` nach Kompilierung behalten |
| `--open` | — | `false` | Ergebnis im Standard-Viewer öffnen |
| `--verbose` | `-v` | `false` | Ausführliches Logging |

### Ausgabe-Pfade

| Szenario | Ergebnis |
|----------|----------|
| `ipmc map.kdl` | `./GOAL-01.svg`, `./GOAL-02.svg`, … |
| `ipmc map.kdl -g GOAL-02` | `./GOAL-02.svg` |
| `ipmc map.kdl -g GOAL-02 -o reports/q3` | `reports/q3.svg` (exakter Name) |
| `ipmc map.kdl -o output/` | `output/GOAL-01.svg`, … |

---

## KDL-Format

Impact Maps folgen einer festen Hierarchie:

```
Goal
├── Metric(s)      — messbare Erfolgskriterien
└── Actor(s)       — wer beeinflusst das Ziel?
    └── Impact(s)  — welches Verhalten ändert der Actor?
        └── Deliverable(s) — was bauen wir dafür?
```

### Schema

```kdl
goal "<Name>" id="<GOAL-ID>" {
    metric "<name>" target="<Zielwert>" desc="<Beschreibung>"

    actor "<Name>" id="<optional>" prio=<1-5> {
        impact "<Name>" id="<optional>" targets="<metric-name>" prio=<n> sp=<story-points> {
            deliverable "<Name>" id="<DEL-ID>" prio=<n> sp=<n>
        }
    }
}
```

### Attribute

| Knoten | Pflicht | Optional |
|--------|---------|----------|
| `goal` | `id` | — |
| `metric` | `target`, `desc` | — |
| `actor` | `prio` | `id` |
| `impact` | `targets`, `prio`, `sp` | `id` |
| `deliverable` | `id`, `prio`, `sp` | — |

- **`targets`** verknüpft einen Impact mit einer `metric` (per Name)
- **`prio`** — niedrigere Zahl = höhere Priorität
- **`sp`** — Story Points (Aufwandsschätzung)

### Beispiel

Siehe [`example.kdl`](example.kdl):

```kdl
goal "Umsatz +40% bei guten Steam-Reviews in Q3/Q4" id="GOAL-02" {
    metric "mrr_growth" target="+40%" desc="Monatlicher Bruttoumsatz"
    metric "steam_rating" target=">=85%" desc="Positive Steam Reviews (Guardrail)"

    actor "Whale" id="whale" prio=1 {
        impact "Gibt Geld für prestigeträchtige Items aus" id="IP-3" targets="mrr_growth" prio=1 sp=13 {
            deliverable "Limitierte animierte Skill-Effekte" id="DEL-201" prio=1 sp=5
        }
    }
}
```

---

## Output

Generierte Graphen nutzen ein konsistentes Farbschema:

| Element | Farbe | Inhalt |
|---------|-------|--------|
| **Goal** | Violett | Name + alle Metriken mit Zielwerten |
| **Actor** | Blau | Name + Priorität |
| **Impact** | Grün | Name + Target-Metrik + Prio/SP |
| **Deliverable** | Orange | Name + Prio/SP |

Layout: links-nach-rechts (`rankdir=LR`), mit Legende oben.

---

## Entwicklung

```bash
# Build
cargo build

# Release-Build
cargo build --release

# Mit Beispieldatei testen (DOT only, kein Graphviz nötig)
cargo run -- example.kdl -f dot --keep-dot -v
```

### Projektstruktur

```
src/
├── main.rs      # CLI-Einstieg, Argument-Parsing, Graphviz-Aufruf
├── parser.rs    # KDL → Datenmodell
├── models.rs    # Goal, Actor, Impact, Deliverable, Metric
└── graphviz.rs  # Datenmodell → DOT mit HTML-Labels
```

---

## Releases

Tags im Format `v*` (z. B. `v0.1.0`) triggern den [Release-Workflow](.github/workflows/release.yml):

- Cross-Compilation für 5 Plattformen
- Automatische GitHub Release mit allen Binaries
- Generierte Release Notes

```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## Fehlerbehebung

| Problem | Lösung |
|---------|--------|
| `Konnte 'dot' nicht starten` | Graphviz installieren, `dot -V` prüfen |
| `Goal 'X' nicht gefunden` | Goal-ID in KDL prüfen (`id="..."`) |
| Graphviz Syntax-Fehler | Mit `--keep-dot` die `.dot`-Datei inspizieren |
| Leere SVG | KDL-Struktur validieren — alle Pflichtattribute gesetzt? |

---

## Lizenz

MIT — siehe [LICENSE](LICENSE).

---

## Weiterführende Links

- [Impact Mapping — Gojko Adzic](https://www.impactmapping.org/)
- [KDL — Document Language](https://kdl.dev/)
- [Graphviz](https://graphviz.org/)
