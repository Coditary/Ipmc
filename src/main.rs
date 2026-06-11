mod graphviz;
mod models;
mod parser;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Ein CLI-Tool, um Impact Maps als Code (KDL) in Graphen zu übersetzen
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Pfad zur KDL-Eingabedatei
    input: String,

    /// Spezifische Goal-ID, die generiert werden soll (z. B. GOAL-02)
    #[arg(short, long)]
    goal: Option<String>,

    /// Zielverzeichnis (Standard) ODER spezifischer Dateiname (wenn -g genutzt wird)
    #[arg(short, long, default_value = ".")]
    out: String,

    /// Ausgabeformat der Datei
    #[arg(short, long, value_enum, default_value_t = Format::Svg)]
    format: Format,

    /// Die generierte .dot Zwischendatei behalten
    #[arg(long)]
    keep_dot: bool,

    /// Generierte Map nach Abschluss automatisch öffnen
    #[arg(long)]
    open: bool,

    /// Ausführliches Logging aktivieren
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Svg,
    Png,
    Dot,
}

impl Format {
    fn extension(&self) -> &'static str {
        match self {
            Format::Svg => "svg",
            Format::Png => "png",
            Format::Dot => "dot",
        }
    }
}

fn main() -> Result<()> {
    // 1. Argumente parsen
    let cli = Cli::parse();

    if cli.verbose {
        println!("🔍 Lese Datei: {}", cli.input);
    }

    // 2. KDL einlesen & parsen
    let kdl_content = fs::read_to_string(&cli.input)
        .unwrap_or_else(|_| panic!("❌ FEHLER: Konnte '{}' nicht finden!", cli.input));
    
    let all_goals = parser::parse_kdl(&kdl_content)?;

    // 3. Nach Goal filtern (falls -g übergeben wurde)
    let goals_to_process: Vec<_> = if let Some(target_goal) = &cli.goal {
        let filtered: Vec<_> = all_goals.into_iter().filter(|g| g.id == *target_goal).collect();
        if filtered.is_empty() {
            println!("❌ Goal '{}' nicht in der KDL-Datei gefunden!", target_goal);
            return Ok(());
        }
        filtered
    } else {
        all_goals
    };

    // 4. Maps generieren
    for goal in goals_to_process {
        if cli.verbose {
            println!("⚙️ Generiere Mindmap für Goal: {}", goal.id);
        }

        // Namenslogik: Wenn -g und -o gesetzt sind, ist -o der exakte Dateiname ohne Endung.
        // Andernfalls ist -o das Zielverzeichnis und die Goal-ID der Dateiname.
        let base_path = if cli.goal.is_some() && cli.out != "." {
            cli.out.clone()
        } else {
            let dir = Path::new(&cli.out);
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
            dir.join(&goal.id).to_string_lossy().into_owned()
        };

        let dot_filename = format!("{}.dot", base_path);
        let ext = cli.format.extension();
        let target_filename = format!("{}.{}", base_path, ext);

        // DOT erzeugen
        let dot_string = graphviz::generate_dot(&goal);
        fs::write(&dot_filename, &dot_string)?;

        // Wenn das Ziel NICHT raw DOT ist, kompilieren wir mit Graphviz
        if cli.format != Format::Dot {
            let output = Command::new("dot")
                .arg(format!("-T{}", ext))
                .arg(&dot_filename)
                .arg("-o")
                .arg(&target_filename)
                .output();

            match output {
                Ok(out) if out.status.success() => {
                    println!("✨ Erfolgreich generiert: {}", target_filename);
                    if !cli.keep_dot {
                        let _ = fs::remove_file(&dot_filename); // Räumt den .dot Code auf
                    }
                }
                Ok(out) => {
                    // Graphviz lief, aber ist mit einem Fehler abgestürzt
                    let err_msg = String::from_utf8_lossy(&out.stderr);
                    println!("❌ Graphviz Syntax-Fehler beim Kompilieren:\n{}", err_msg);
                    println!("💡 Tipp: Führe das Tool mit --keep-dot aus, um dir die defekte .dot Datei anzusehen.");
                }
                Err(e) => println!("❌ Konnte den Graphviz-Compiler ('dot') nicht starten: {}", e),
            }
        } else {
            println!("✨ Erfolgreich generiert: {}", target_filename);
        }

        // 5. Öffnen (falls Flag gesetzt)
        if cli.open {
            if cli.verbose {
                println!("📂 Öffne Datei...");
            }
            let _ = open::that(&target_filename);
        }
    }

    Ok(())
}
