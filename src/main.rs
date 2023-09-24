use colored::*;
use loading::Loading;
use piechart::{Chart, Color, Data};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::{env, str, thread};

/// Executes a command in PowerShell and returns the output as a string.
///
/// # Arguments
///
/// * `command` - A string slice that holds the command to be executed.
///
/// # Returns
///
/// A string that holds the output of the executed command.
fn run_command(command: &str) -> String {
    let output: std::process::Output = Command::new("powershell.exe")
        .args(&["-Command", &command])
        .output()
        .expect("Failed to execute command");

    str::from_utf8(&output.stdout)
        .expect("Invalid UTF-8")
        .to_string()
}

fn get_header() {
    let text = r#"
   ___ _ _       ___          _        __ _        _       
  / _ (_) |_    / __\___   __| | ___  / _\ |_ __ _| |_ ___ 
 / /_\/ | __|  / /  / _ \ / _` |/ _ \ \ \| __/ _` | __/ __|
/ /_\\| | |_  / /__| (_) | (_| |  __/ _\ \ || (_| | |_\__ \
\____/|_|\__| \____/\___/ \__,_|\___| \__/\__\__,_|\__|___/
                                             
                                             author: AbianS                                                        
"#.green();
    println!("{}", text);
}

/// Clears the terminal screen.
fn clear_terminal() {
    let _ = Command::new("cmd").arg("/c").arg("cls").status();
}

/// Returns a vector of unique authors in the Git repository.
///
/// This function runs a Git command to retrieve the list of unique authors in the repository.
/// The command is executed using the `run_command` function and the output is parsed to
/// extract the author names. The resulting vector contains the unique author names.
///
/// # Returns
///
/// A vector of strings representing the unique authors in the Git repository.
fn get_unique_authors() -> Vec<String> {
    let command: &str = "git log --format='%aN' | Sort-Object -Unique";
    let stdout: String = run_command(command);
    let authors: Vec<String> = stdout
        .trim()
        .split('\n')
        .map(|author| author.trim().to_string())
        .collect();
    authors
}

/// This function takes an author name as input and returns a tuple containing the author name, the number of lines inserted by the author, and the number of files changed by the author.
/// It uses the `git log` command to get the commit history of the repository and extracts the number of files changed, lines inserted and lines deleted by the given author using regular expressions.
///
/// # Arguments
///
/// * `author` - A string slice that holds the name of the author whose statistics are to be calculated.
///
/// # Returns
///
/// A tuple containing the author name, the number of lines inserted by the author, and the number of files changed by the author.
fn get_git_stats(author: &str) -> (String, i32, i32) {
    let command: String = format!("git log --shortstat --author=\"{}\"", author);
    let stdout = run_command(&command);

    let mut files_changed: i32 = 0;
    let mut lines_inserted: i32 = 0;
    let mut lines_deleted: i32 = 0;

    let re: regex::Regex =
        regex::Regex::new(r"(\d+) files? changed, (\d+) insertions\(\+\), (\d+) deletions\(-\)")
            .unwrap();

    for line in stdout.lines() {
        if let Some(captures) = re.captures(line) {
            files_changed += captures[1].parse::<i32>().unwrap();
            lines_inserted += captures[2].parse::<i32>().unwrap();
            lines_deleted += captures[3].parse::<i32>().unwrap();
        }
    }

    (author.to_string(), lines_inserted, files_changed) // Cambiado el orden de retorno para reflejar las líneas insertadas y los archivos cambiados
}

fn main() {
    // Gets the current directory and sets it as the working directory.
    let current_dir: std::path::PathBuf =
        env::current_dir().expect("Failed to get current directory");
    env::set_current_dir(current_dir).expect("Failed to set current directory");

    clear_terminal();
    get_header();

    // Checks if the current directory is a git repository by running the `git rev-parse --is-inside-work-tree` command.
    // Returns a boolean indicating whether the current directory is a git repository or not.
    let git_repo_exists: bool = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .stdout(Stdio::null()) // Redirect standard output to NUL
        .stderr(Stdio::null()) // Redirect error output to NUL
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    // Checks if a Git repository exists in the current directory. If not, it prints an error message and exits the program.
    if !git_repo_exists {
        let text1: ColoredString = r#"[!]"#.red();
        let text2: ColoredString = r#"Git repository not found in the current directory."#.blue();
        println!("{} {}", text1, text2);
        return; // Salir del programa
    }

    let loading: Loading = Loading::default();
    loading.text("Getting git stats...".blue());

    let authors: Vec<String> = get_unique_authors();
    let mut author_stats: HashMap<String, (f32, Color)> = HashMap::new();

    let colors = vec![
        Color::RGB(255, 0, 0),
        Color::RGB(0, 255, 0),
        Color::RGB(0, 0, 255),
        Color::RGB(255, 255, 0),
        Color::RGB(255, 0, 255),
        Color::RGB(0, 255, 255),
        Color::RGB(128, 0, 0),
        Color::RGB(0, 128, 0),
        Color::RGB(0, 0, 128),
        Color::RGB(128, 128, 0),
    ];

    // Creates a vector of threads, where each thread calculates the git statistics for a given author.
    // The function returns a vector of handles to the threads.
    let handles: Vec<_> = authors
        .iter()
        .enumerate()
        .map(|(index, author)| {
            let color = colors[index % colors.len()];
            let author_clone = author.clone();

            thread::spawn(move || {
                let (author_name, lines_inserted, files_changed): (String, i32, i32) =
                    get_git_stats(&author_clone);
                (author_name.clone(), (lines_inserted as f32, color))
            })
        })
        .collect();

    // Iterate over the handles and insert the author name and stats into the author_stats HashMap.
    for handle in handles {
        if let Ok((author_name, stats)) = handle.join() {
            author_stats.insert(author_name, stats);
        }
    }

    loading.end();

    // Maps the author statistics to a vector of `Data` structs.
    //
    // # Arguments
    //
    // * `author_stats` - A reference to a `HashMap` containing the author statistics.
    //
    // # Returns
    //
    // A vector of `Data` structs, where each struct contains the author's name, number of lines inserted,
    // color and fill character.
    let data: Vec<Data> = author_stats
        .iter()
        .map(|(author, (lines_inserted, color))| Data {
            label: author.to_string(),
            value: *lines_inserted,
            color: Some((*color).into()),
            fill: '•',
        })
        .collect();

    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);
}
