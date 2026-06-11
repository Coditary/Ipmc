# Impact Map CLI (`ipmc`)

**Impact maps as code: versionable, diffable, renderable.**

`ipmc` is a CLI tool that turns [Impact Maps](https://www.impactmapping.org/) from a structured [KDL](https://kdl.dev/) file into high-quality, color-coded graphs. Instead of maintaining mind maps in whiteboard tools, you describe goals, metrics, actors, impacts, and deliverables as code and generate SVG, PNG, or DOT files from them.

```
KDL file  →  Parser  →  Graphviz DOT  →  SVG / PNG
```

---

## Why impact maps as code?

Impact mapping connects **business goals** with **measurable metrics**, **actors**, their **behavior (impacts)**, and concrete **deliverables**. Classic mind map tools are hard to version and difficult to review collaboratively.

With `ipmc` you get:

- **Git-friendly** impact maps as text files, diffable and reviewable
- **Automatable** via CI/CD, scripts, and batch rendering
- **Consistent** layout and color scheme per goal
- **Visible prioritization** with priority and story points shown directly on nodes

---

## Features

| Feature | Description |
|---------|-------------|
| KDL parser | Declarative format for goals, metrics, actors, impacts, deliverables |
| Multi-goal | One file, many goals: render individually or all at once |
| Formats | SVG (default), PNG, raw DOT |
| Styled output | Color-coded HTML labels with legend, metrics, and priorities |
| Auto-open | Open the generated map in your default viewer |
| Cross-platform | Linux, macOS, Windows with prebuilt binaries via GitHub Releases |

---

## Prerequisites

| Dependency | When needed | Installation |
|--------------|-------------|--------------|
| **Rust** (1.70+) | Build from source | [rustup.rs](https://rustup.rs/) |
| **Graphviz** (`dot`) | SVG/PNG export | `sudo dnf install graphviz` (Fedora) · `brew install graphviz` (macOS) · [graphviz.org](https://graphviz.org/download/) |

> Only `--format dot` requires no Graphviz. The `.dot` file is written directly.

---

## Installation

### Prebuilt binary (recommended)

Download the appropriate binary from [GitHub Releases](https://github.com/Coditary/Ipmc/releases) (triggered by `v*` tags):

```bash
# Example: Linux x86_64
curl -LO https://github.com/Coditary/Ipmc/releases/latest/download/ipmc-x86_64-unknown-linux-gnu
chmod +x ipmc-x86_64-unknown-linux-gnu
sudo mv ipmc-x86_64-unknown-linux-gnu /usr/local/bin/ipmc
```

Available targets: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc`

### From source

```bash
git clone https://github.com/Coditary/Ipmc.git
cd Ipmc
cargo build --release
# Binary: target/release/impact-map-cli
```

---

## Quick start

```bash
# Render all goals from example.kdl as SVG
cargo run --release -- example.kdl

# A specific goal
cargo run --release -- example.kdl -g GOAL-02

# As PNG, with auto-open and verbose logging
cargo run --release -- example.kdl -g GOAL-02 -f png --open -v
```

Output: `GOAL-02.svg` (or `GOAL-02.png`) in the current directory.

---

## CLI reference

```
ipmc <INPUT> [OPTIONS]
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--goal` | `-g` | (none) | Render only this goal (e.g. `GOAL-02`) |
| `--out` | `-o` | `.` | Output directory **or** exact filename (without extension) when using `-g` |
| `--format` | `-f` | `svg` | `svg` · `png` · `dot` |
| `--keep-dot` | | `false` | Keep intermediate `.dot` file after compilation |
| `--open` | | `false` | Open result in default viewer |
| `--verbose` | `-v` | `false` | Verbose logging |

### Output paths

| Scenario | Result |
|----------|--------|
| `ipmc map.kdl` | `./GOAL-01.svg`, `./GOAL-02.svg`, … |
| `ipmc map.kdl -g GOAL-02` | `./GOAL-02.svg` |
| `ipmc map.kdl -g GOAL-02 -o reports/q3` | `reports/q3.svg` (exact name) |
| `ipmc map.kdl -o output/` | `output/GOAL-01.svg`, … |

---

## KDL format

Impact maps follow a fixed hierarchy:

```
Goal
├── Metric(s)      measurable success criteria
└── Actor(s)       who influences the goal?
    └── Impact(s)  what behavior does the actor change?
        └── Deliverable(s) what do we build for that?
```

### Schema

```kdl
goal "<Name>" id="<GOAL-ID>" {
    metric "<name>" target="<target value>" desc="<description>"

    actor "<Name>" id="<optional>" prio=<1-5> {
        impact "<Name>" id="<optional>" targets="<metric-name>" prio=<n> sp=<story-points> {
            deliverable "<Name>" id="<DEL-ID>" prio=<n> sp=<n>
        }
    }
}
```

### Attributes

| Node | Required | Optional |
|--------|---------|----------|
| `goal` | `id` | |
| `metric` | `target`, `desc` | |
| `actor` | `prio` | `id` |
| `impact` | `targets`, `prio`, `sp` | `id` |
| `deliverable` | `id`, `prio`, `sp` | |

- **`targets`** links an impact to a `metric` (by name)
- **`prio`**: lower number = higher priority
- **`sp`**: story points (effort estimate)

### Example

See [`example.kdl`](example.kdl):

```kdl
goal "40% revenue growth with good Steam reviews in Q3/Q4" id="GOAL-02" {
    metric "mrr_growth" target="+40%" desc="Monthly gross revenue"
    metric "steam_rating" target=">=85%" desc="Positive Steam reviews (guardrail)"

    actor "Whale" id="whale" prio=1 {
        impact "Spends money on prestigious items" id="IP-3" targets="mrr_growth" prio=1 sp=13 {
            deliverable "Limited animated skill effects" id="DEL-201" prio=1 sp=5
        }
    }
}
```

---

## Output

Generated graphs use a consistent color scheme:

| Element | Color | Content |
|---------|-------|---------|
| **Goal** | Purple | Name + all metrics with target values |
| **Actor** | Blue | Name + priority |
| **Impact** | Green | Name + target metric + prio/SP |
| **Deliverable** | Orange | Name + prio/SP |

Layout: left-to-right (`rankdir=LR`), with legend at the top.

---

## Development

```bash
# Build
cargo build

# Release build
cargo build --release

# Test with example file (DOT only, no Graphviz needed)
cargo run -- example.kdl -f dot --keep-dot -v
```

### Project structure

```
src/
├── main.rs      # CLI entry, argument parsing, Graphviz invocation
├── parser.rs    # KDL → data model
├── models.rs    # Goal, Actor, Impact, Deliverable, Metric
└── graphviz.rs  # Data model → DOT with HTML labels
```

---

## Releases

Tags in the format `v*` (e.g. `v0.1.0`) trigger the [release workflow](.github/workflows/release.yml):

- Cross-compilation for 5 platforms
- Automatic GitHub release with all binaries
- Generated release notes

```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `Could not start 'dot'` | Install Graphviz, verify with `dot -V` |
| `Goal 'X' not found` | Check goal ID in KDL (`id="..."`) |
| Graphviz syntax error | Inspect the `.dot` file with `--keep-dot` |
| Empty SVG | Validate KDL structure: are all required attributes set? |

---

## License

MIT. See [LICENSE](LICENSE).

---

## Further reading

- [Impact Mapping by Gojko Adzic](https://www.impactmapping.org/)
- [KDL Document Language](https://kdl.dev/)
- [Graphviz](https://graphviz.org/)
