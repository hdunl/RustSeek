mod search;

use search::Search;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use dialoguer::{theme::ColorfulTheme, Select, Input};
use indicatif::{ProgressBar, ProgressStyle};
use terminal_emoji::Emoji;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .unwrap();
    } else {
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn display_banner(stdout: &mut StandardStream) {
    let banner = r#"
         _      _                  _          _          _            _            _            _
        /\ \   /\_\               / /\       /\ \       / /\         /\ \         /\ \         /\_\
       /  \ \ / / /         _    / /  \      \_\ \     / /  \       /  \ \       /  \ \       / / /  _
      / /\ \ \\ \ \__      /\_\ / / /\ \__   /\__ \   / / /\ \__   / /\ \ \     / /\ \ \     / / /  /\_\
     / / /\ \_\\ \___\    / / // / /\ \___\ / /_ \ \ / / /\ \___\ / / /\ \_\   / / /\ \_\   / / /__/ / /
    / / /_/ / / \__  /   / / / \ \ \ \/___// / /\ \ \\ \ \ \/___// /_/_ \/_/  / /_/_ \/_/  / /\_____/ /
   / / /__\/ /  / / /   / / /   \ \ \     / / /  \/_/ \ \ \     / /____/\    / /____/\    / /\_______/
  / / /_____/  / / /   / / /_    \ \ \   / / /    _    \ \ \   / /\____\/   / /\____\/   / / /\ \ \
 / / /\ \ \   / / /___/ / //_/\__/ / /  / / /    /_/\__/ / /  / / /______  / / /______  / / /  \ \ \
/ / /  \ \ \ / / /____\/ / \ \/___/ /  /_/ /     \ \/___/ /  / / /_______\/ / /_______\/ / /    \ \ \
\/_/    \_\/ \/_________/   \_____\/   \_\/       \_____\/   \/__________/\/__________/\/_/      \_\_\

    "#;

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true)).unwrap();
    writeln!(stdout, "{}", banner).unwrap();
    stdout.reset().unwrap();
}

fn main() {
    let mut search_term = String::new();
    let mut search_directory = String::new();
    let mut search_zip = false;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    loop {
        clear_screen();
        display_banner(&mut stdout);

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true)).unwrap();
        writeln!(&mut stdout, "\n{} Current Settings:", Emoji::new("🔍", "[Search]")).unwrap();
        stdout.reset().unwrap();

        writeln!(&mut stdout, "{} Search Term: {}", Emoji::new("📝", "[Term]"), if search_term.is_empty() { "Not set" } else { &search_term }).unwrap();
        writeln!(&mut stdout, "{} Search Directory: {}", Emoji::new("📁", "[Directory]"), if search_directory.is_empty() { "Not set" } else { &search_directory }).unwrap();
        writeln!(&mut stdout, "{} Search ZIP files: {}", Emoji::new("🗜️", "[ZIP]"), if search_zip { "Enabled" } else { "Disabled" }).unwrap();

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true)).unwrap();
        writeln!(&mut stdout, "\n{} Choose an option:", Emoji::new("🚀", "[Options]")).unwrap();
        stdout.reset().unwrap();

        let options = vec![
            format!("{} Start Search", Emoji::new("🔎", "[Search]")),
            format!("{} Change Search Term", Emoji::new("✏️", "[Edit]")),
            format!("{} Change Directory", Emoji::new("📂", "[Folder]")),
            format!("{} Toggle ZIP Search", Emoji::new("🔄", "[Toggle]")),
            format!("{} Exit", Emoji::new("❌", "[Exit]")),
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                if search_term.is_empty() || search_directory.is_empty() {
                    display_error(&mut stdout, &format!("{} Please set search term and directory before starting the search.", Emoji::new("⚠️", "[Warning]")));
                    prompt("\nPress Enter to continue...");
                    continue;
                }
                perform_search(&search_term, &search_directory, search_zip, &mut stdout);
            },
            1 => search_term = Input::new().with_prompt("Enter the search term").interact_text().unwrap(),
            2 => search_directory = Input::new().with_prompt("Enter the directory to search in").interact_text().unwrap(),
            3 => {
                search_zip = !search_zip;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)).set_bold(true)).unwrap();
                writeln!(&mut stdout, "\n{} ZIP search is now {}", Emoji::new("🔄", "[Toggle]"), if search_zip { "enabled" } else { "disabled" }).unwrap();
                stdout.reset().unwrap();
                prompt("\nPress Enter to continue...");
            },
            4 => {
                display_goodbye(&mut stdout);
                break;
            },
            _ => unreachable!(),
        }
    }
}

fn perform_search(search_term: &str, search_directory: &str, search_zip: bool, stdout: &mut StandardStream) {
    clear_screen();
    display_banner(stdout);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true)).unwrap();
    writeln!(stdout, "\n{} Searching for '{}' in '{}'...", Emoji::new("🔍", "Search"), search_term, search_directory).unwrap();
    writeln!(stdout, "{} ZIP search is {}", Emoji::new("🗜️", "ZIP"), if search_zip { "enabled" } else { "disabled" }).unwrap();
    stdout.reset().unwrap();

    let start_time = Instant::now();
    let mut search = Search::new(search_directory, Some(search_term), search_zip);
    let results: Vec<String> = search.by_ref().collect();
    let duration = start_time.elapsed();

    display_results(stdout, &results);
    display_summary(stdout, &search, duration);

    prompt("\nPress Enter to return to the main menu...");
}

fn display_results(stdout: &mut StandardStream, results: &[String]) {
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true)).unwrap();
    writeln!(stdout, "\n{} Search Results:", Emoji::new("📊", "Results")).unwrap();
    stdout.reset().unwrap();

    let pb = ProgressBar::new(results.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
        .unwrap()
        .progress_chars("#>-"));

    for result in results {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
        writeln!(stdout, "- {}", result).unwrap();
        pb.inc(1);
        std::thread::sleep(Duration::from_millis(50)); // Simulate processing time
    }

    pb.finish_with_message("Done");
}

fn display_summary(stdout: &mut StandardStream, search: &Search, duration: Duration) {
    let num_results = *search.num_results.lock().unwrap();
    let num_files_processed = *search.num_files_processed.lock().unwrap();
    let files_per_second = num_files_processed as f64 / duration.as_secs_f64();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true)).unwrap();
    writeln!(stdout, "\n╭────────────────────────────────────────╮").unwrap();
    writeln!(stdout, "│             Search Summary             │").unwrap();
    writeln!(stdout, "╰────────────────────────────────────────╯").unwrap();
    stdout.reset().unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)).set_bold(true)).unwrap();
    writeln!(stdout, "{} Time taken: {:.2?} seconds", Emoji::new("⏱️", "Time"), duration).unwrap();
    writeln!(stdout, "{} Number of results found: {}", Emoji::new("🎯", "Results"), num_results).unwrap();
    writeln!(stdout, "{} Number of files processed: {}", Emoji::new("📄", "Files"), num_files_processed).unwrap();
    writeln!(stdout, "{} Files per second processed: {:.2}", Emoji::new("⚡", "Speed"), files_per_second).unwrap();

    stdout.reset().unwrap();
}

fn display_error(stdout: &mut StandardStream, message: &str) {
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
    writeln!(stdout, "{}", message).unwrap();
    stdout.reset().unwrap();
}

fn display_goodbye(stdout: &mut StandardStream) {
    clear_screen();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true)).unwrap();
    writeln!(stdout, r#"
   ______                 ______               __
  / ____/___  ____  ____/ / __ )__  _____     / /
 / / __/ __ \/ __ \/ __  / __  / / / / _ \   / /
/ /_/ / /_/ / /_/ / /_/ / /_/ / /_/ /  __/  /_/
\____/\____/\____/\__,_/_____/\__, /\___/  (_)
                             /____/
    "#).unwrap();
    stdout.reset().unwrap();
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