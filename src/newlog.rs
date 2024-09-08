use chrono::Local;
use edit::edit_file;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tempfile::tempdir;
use clap::ArgMatches;

fn format_title(title: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap(); // Regex for non-alphanumeric characters
    re.replace_all(&title.to_lowercase(), "-").to_string()
}

pub fn new_log(_args: &ArgMatches, logs_dir: &str) -> io::Result<()> {
    let mut logs_dir = PathBuf::from(logs_dir);
    // Get the current date in YYYY-MM-DD format
    let mut date = Local::now().format("%Y-%m-%d").to_string();

    // Create a temporary directory
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path().join("template.md");

    // Create and write to the temporary file
    let mut file = File::create(&temp_path)?;
    writeln!(file, "title=")?;
    writeln!(file, "date={}", date)?;

    // Open the file for editing
    edit_file(&temp_path)?;

    // Read the title and date from the file
    let file = File::open(&temp_path)?;
    let reader = io::BufReader::new(file);
    let mut title = String::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("title=") {
            title = line.trim_start_matches("title=").trim().to_string();
        }
        if line.starts_with("date=") {
            date = line.trim_start_matches("date=").trim().to_string();
            break;
        }
    }

    // Format the title
    let formatted_title = format_title(&title);

    // Check for existing files with the same date
    let mut create_subdir = false;
    let mut files_to_move = Vec::new();

    for entry in fs::read_dir(&logs_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with(&format!("{}-", date)) {
                        create_subdir = true;
                        files_to_move.push(path);
                    }
                }
            }
        }
    }

    // Create subdirectory if needed
    if create_subdir {
        let subdir_path = logs_dir.join(format!("{}", date));
        println!("{:?}", subdir_path);
        fs::create_dir_all(&subdir_path)?;

        // Move existing files to the subdirectory
        for file in files_to_move {
            let file_name = file.file_name().unwrap();
            let new_path = subdir_path.join(file_name);
            fs::rename(&file, &new_path)?;
        }

        logs_dir = subdir_path;
    }

    let logs_path = logs_dir.join(format!("{}-{}.md", date, formatted_title));

    // Move the file to the permanent directory
    fs::copy(&temp_path, &logs_path)?;
    fs::remove_file(&temp_path)?; // Remove the temporary file

    Ok(())
}

