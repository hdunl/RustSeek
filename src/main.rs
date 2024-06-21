mod search;

use search::Search;
use std::io::{self, Write};
use std::time::Instant;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let search_term = prompt("Enter the search term: ");
    let search_directory = prompt("Enter the directory to search in: ");
    let search_zip = prompt("Include ZIP archives in the search? (yes/no): ").to_lowercase() == "yes";

    let start_time = Instant::now();
    let mut search = Search::new(search_directory, Some(&search_term), search_zip);
    let results: Vec<String> = search.by_ref().collect();
    let duration = start_time.elapsed();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true)).unwrap();
    writeln!(&mut stdout, "\nSearch Results:").unwrap();
    stdout.reset().unwrap();

    for result in results {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
        writeln!(&mut stdout, "- {}", result).unwrap();
    }

    let num_results = *search.num_results.lock().unwrap();
    let num_files_processed = *search.num_files_processed.lock().unwrap();
    let files_per_second = num_files_processed as f64 / duration.as_secs_f64();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true)).unwrap();
    writeln!(&mut stdout, "\n╭────────────────────────────────────────╮").unwrap();
    writeln!(&mut stdout, "│             Search Summary             │").unwrap();
    writeln!(&mut stdout, "╰────────────────────────────────────────╯").unwrap();
    stdout.reset().unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)).set_bold(true)).unwrap();
    writeln!(&mut stdout, "Time taken: {:.2?} seconds", duration).unwrap();
    writeln!(&mut stdout, "Number of results found: {}", num_results).unwrap();
    writeln!(&mut stdout, "Number of files processed: {}", num_files_processed).unwrap();
    writeln!(&mut stdout, "Files per second processed: {:.2}", files_per_second).unwrap();

    stdout.reset().unwrap();

    prompt("\nPress Enter to exit...");
}

fn prompt(message: &str) -> String {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true)).unwrap();
    print!("{}", message);
    io::stdout().flush().unwrap();
    stdout.reset().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
