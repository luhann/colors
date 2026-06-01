use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::ExitCode,
};

use clap::{Parser, Subcommand};
use colored::*;
use rayon::prelude::*;

#[derive(Parser)]
#[command(name = "colorctl")]
#[command(about = "Unified color manipulation and checking CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Determine black-pixel percentage for an image or directory.
    Black {
        /// File or directory path
        path: String,

        /// Process every image in a directory
        #[arg(short, long, default_value_t = false)]
        directory: bool,
    },

    /// Convert HEX colors into different output formats.
    Hex {
        /// Hex color code (e.g. #FF0000, 0xAABBCC, FFAABBCC)
        hex_color: Option<String>,

        /// Output format
        #[arg(short, long, value_enum, default_value_t = hextorgb::OutputFormat::Standard)]
        format: hextorgb::OutputFormat,

        /// Interactive mode
        #[arg(short, long, default_value_t = false)]
        interactive: bool,

        /// Show color preview (requires true-color terminal)
        #[arg(short, long, default_value_t = false)]
        preview: bool,
    },

    /// Convert RGB colors into different output formats.
    Rgb {
        /// RGB input (e.g. 255,0,0 or 255,0,0,0.5)
        rgb_color: Option<String>,

        /// Output format
        #[arg(short, long, value_enum, default_value_t = rgbtohex::OutputFormat::Hex)]
        format: rgbtohex::OutputFormat,

        /// Interactive mode
        #[arg(short, long, default_value_t = false)]
        interactive: bool,

        /// Show color preview (requires true-color terminal)
        #[arg(short, long, default_value_t = false)]
        preview: bool,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{} {}", "Error:".red().bold(), message);
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Black { path, directory } => run_black(&path, directory),
        Command::Hex {
            hex_color,
            format,
            interactive,
            preview,
        } => run_hex(hex_color, format, interactive, preview),
        Command::Rgb {
            rgb_color,
            format,
            interactive,
            preview,
        } => run_rgb(rgb_color, format, interactive, preview),
    }
}

fn run_black(path: &str, directory: bool) -> Result<(), String> {
    let path = Path::new(path);

    if directory {
        if !path.is_dir() {
            return Err("The provided path is not a directory.".to_string());
        }

        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory '{}': {}", path.display(), e))?;

        let image_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|file_path| black::is_image_file(file_path))
            .collect();

        if image_files.is_empty() {
            println!("No image files found in the directory.");
            return Ok(());
        }

        let failures = image_files
            .par_iter()
            .map(|file_path| match black::black(file_path, false) {
                Ok(percentage) => {
                    print_black_result(file_path, percentage);
                    0usize
                }
                Err(e) => {
                    eprintln!("{} {}: {}", "Error:".red().bold(), file_path.display(), e);
                    1usize
                }
            })
            .sum::<usize>();

        if failures > 0 {
            return Err(format!("{} image(s) could not be processed.", failures));
        }

        return Ok(());
    }

    if !black::is_image_file(path) {
        return Err("The provided path does not point to a valid image file.".to_string());
    }

    let percentage = black::black(path, true)
        .map_err(|e| format!("Failed to process '{}': {}", path.display(), e))?;
    print_black_result(path, percentage);
    Ok(())
}

fn print_black_result(path: &Path, percentage: f64) {
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    println!("{}: {:.2}%", filename.green(), percentage);
}

fn run_hex(
    hex_color: Option<String>,
    format: hextorgb::OutputFormat,
    interactive: bool,
    preview: bool,
) -> Result<(), String> {
    if interactive {
        run_hex_interactive(format, preview)?;
        return Ok(());
    }

    let hex_color = hex_color
        .as_deref()
        .ok_or_else(|| "Please provide a hex color or use --interactive mode.".to_string())?;

    let result = hextorgb::convert_with_format(hex_color.trim(), &format, preview)?;
    println!("{}", result);
    Ok(())
}

fn run_hex_interactive(format: hextorgb::OutputFormat, preview: bool) -> Result<(), String> {
    println!(
        "{}",
        "Hex Converter - Interactive Mode".bright_cyan().bold()
    );
    println!("{}", "Enter hex colors (type 'quit' to exit)".dimmed());

    loop {
        print!("{} ", "hex>".bright_green().bold());
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {}", e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("{}", "Goodbye!".bright_cyan());
            break;
        }

        match hextorgb::convert_with_format(input, &format, preview) {
            Ok(result) => println!("  {}", result),
            Err(e) => println!("  {} {}", "Error:".red().bold(), e),
        }
    }

    Ok(())
}

fn run_rgb(
    rgb_color: Option<String>,
    format: rgbtohex::OutputFormat,
    interactive: bool,
    preview: bool,
) -> Result<(), String> {
    if interactive {
        run_rgb_interactive(format, preview)?;
        return Ok(());
    }

    let rgb_color = rgb_color
        .as_deref()
        .ok_or_else(|| "Please provide an RGB color or use --interactive mode.".to_string())?;

    let result = rgbtohex::convert_with_format(rgb_color.trim(), &format, preview)?;
    println!("{}", result);
    Ok(())
}

fn run_rgb_interactive(format: rgbtohex::OutputFormat, preview: bool) -> Result<(), String> {
    println!(
        "{}",
        "RGB Converter - Interactive Mode".bright_cyan().bold()
    );
    println!(
        "{}",
        "Enter RGB colors as r,g,b or r,g,b,a (type 'quit' to exit)".dimmed()
    );

    loop {
        print!("{} ", "rgb>".bright_green().bold());
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {}", e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("{}", "Goodbye!".bright_cyan());
            break;
        }

        match rgbtohex::convert_with_format(input, &format, preview) {
            Ok(result) => println!("  {}", result),
            Err(e) => println!("  {} {}", "Error:".red().bold(), e),
        }
    }

    Ok(())
}
