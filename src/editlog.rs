use crate::readlog::choose_file_fuzzy;
use clap::ArgMatches;
use std::error::Error;
use tempfile::tempdir;
use walkdir::WalkDir;

pub fn edit_log(_args: &ArgMatches, logs_dir: &str) -> Result<(), Box<dyn Error>> {
    let chosen_file_name = choose_file_fuzzy(&logs_dir)?;
    let _ = edit_chosen_file(&logs_dir, chosen_file_name)?;
    Ok(())
}

fn edit_chosen_file(logs_dir: &str, chosen_file_name: String) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(logs_dir) {
        match entry {
            Ok(file) => {
                if chosen_file_name.as_str() == file.file_name()
                    .to_str().expect("Failed to convert filename to string") {
                    let chosen_file_path = file.path();
                    let temp_dir = tempdir()?;
                }
            },
            Err(_e) => {}
        }
    }
    Ok(())
}
