mod compiler;
mod lexer;
mod parser;
mod traits;

use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

use compiler::LolCompiler;
use traits::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        process::exit(1);
    }
    let input_path = &args[1];

    // the input file must end in .lol
    if !input_path.to_lowercase().ends_with(".lol") {
        eprintln!("Usage error: the input file must have a .lol extension");
        process::exit(1);
    }

    // read the file
    let source = match fs::read_to_string(input_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Could not read '{}': {}", input_path, e);
            process::exit(1);
        }
    };

    // compile
    let mut lol = LolCompiler::new();
    lol.compile(&source);

    // same folder and name as the input but ending in .html
    let output_path = Path::new(input_path).with_extension("html");
    if let Err(e) = fs::write(&output_path, lol.get_html()) {
        eprintln!("Could not write '{}': {}", output_path.display(), e);
        process::exit(1);
    }
    println!("Compiled {} -> {}", input_path, output_path.display());

    open_in_browser(&output_path);
}

// ask to open in respective computers default browser
fn open_in_browser(path: &Path) {
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(path).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", ""]).arg(path).status()
    } else {
        Command::new("xdg-open").arg(path).status()
    };
    if result.is_err() {
        eprintln!("Note: could not open the browser automatically.");
    }
}
