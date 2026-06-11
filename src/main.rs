mod graphviz;
mod models;
mod parser;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: String,

    #[arg(short, long)]
    goal: Option<String>,

    #[arg(short, long, default_value = ".")]
    out: String,

    #[arg(short, long, value_enum, default_value_t = Format::Svg)]
    format: Format,

    #[arg(long)]
    keep_dot: bool,

    #[arg(long)]
    open: bool,

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
    let cli = Cli::parse();

    if cli.verbose {
        println!("Reading file: {}", cli.input);
    }

    let kdl_content = fs::read_to_string(&cli.input)
        .unwrap_or_else(|_| panic!("ERROR: Could not find '{}'!", cli.input));
    
    let all_goals = parser::parse_kdl(&kdl_content)?;

    let goals_to_process: Vec<_> = if let Some(target_goal) = &cli.goal {
        let filtered: Vec<_> = all_goals.into_iter().filter(|g| g.id == *target_goal).collect();
        if filtered.is_empty() {
            println!("ERROR: Goal '{}' not found in the KDL file!", target_goal);
            return Ok(());
        }
        filtered
    } else {
        all_goals
    };

    for goal in goals_to_process {
        if cli.verbose {
            println!("Generating mindmap for goal: {}", goal.id);
        }

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

        let dot_string = graphviz::generate_dot(&goal);
        fs::write(&dot_filename, &dot_string)?;

        if cli.format != Format::Dot {
            let output = Command::new("dot")
                .arg(format!("-T{}", ext))
                .arg(&dot_filename)
                .arg("-o")
                .arg(&target_filename)
                .output();

            match output {
                Ok(out) if out.status.success() => {
                    println!("Successfully generated: {}", target_filename);
                    if !cli.keep_dot {
                        let _ = fs::remove_file(&dot_filename);
                    }
                }
                Ok(out) => {
                    let err_msg = String::from_utf8_lossy(&out.stderr);
                    println!("Graphviz syntax error during compilation:\n{}", err_msg);
                    println!("Tip: Run the tool with --keep-dot to inspect the broken .dot file.");
                }
                Err(e) => println!("Could not start Graphviz compiler ('dot'): {}", e),
            }
        } else {
            println!("Successfully generated: {}", target_filename);
        }

        if cli.open {
            if cli.verbose {
                println!("Opening file...");
            }
            let _ = open::that(&target_filename);
        }
    }

    Ok(())
}
