use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;
#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn print_usage() {
    eprintln!(
        "{} - zmienia wszystkie wystąpienia łańcucha na inny",
        "quickreplace".green()
    );
    eprintln!("Sposób użycia: quickreplace <cel> <zamiennik> <plik_wejściowy> <plik_wyjściowy>");
}
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} nieprawidłowa liczba argumentów: oczekiwano 4, znaleziono {}.",
            "Błąd:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}
fn main() {
    let args = parse_args();
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} nie udało się odczytać zawartości pliku '{}': {:?}",
                "Błąd".red().bold(),
                &args.filename,
                e
            );
            std::process::exit(1)
        }
    };
    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} nie udało się zastąpić tekstu: {:?}",
                "Błąd".red().bold(),
                e
            );
            std::process::exit(1);
        }
    };
    match fs::write(&args.output, replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} nie udało się zapisać danych do pliku '{}': {:?}",
                "Błąd".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    }
    println!("{:?}", args);
}
