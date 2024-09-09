use fzf_wrapped::Fzf;
use std::error::Error;
use clap::ArgMatches;
use walkdir::WalkDir;
use edit::edit_file;
use tempfile::tempdir;
use std::fs;

pub fn read_log(_args: &ArgMatches, logs_dir: &str) -> Result<(), Box<dyn Error>> {
    let chosen_file_name = choose_file_fuzzy(&logs_dir)?;
    let _ = open_chosen_file_readonly(&logs_dir, chosen_file_name);

    Ok(())
}

pub fn choose_file_fuzzy(logs_dir: &str) -> Result<String, Box<dyn Error>> {
    let mut file_names: Vec<String> = Vec::new();

    for file in WalkDir::new(logs_dir) {
        let file_string = file?.file_name()
            .to_str().expect("Failed to convert filename to string").to_string();
        if file_string.ends_with(".md") {
            file_names.push(file_string)
        }
    }

    let mut fzf = Fzf::default();
    fzf.run()?;
    fzf.add_items(file_names)?;
    let chosen_file_name = fzf.output().expect("Failed to get fzf output");
    Ok(chosen_file_name)
}

fn open_chosen_file_readonly(logs_dir: &str, chosen_file_name: String) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(logs_dir) {
        match entry {
            Ok(file) => {
                if chosen_file_name.as_str() == file.file_name()
                    .to_str().expect("Failed to convert filename to string") {
                    let chosen_file_path = file.path();
                    let temp_dir = tempdir()?;
                    let temp_path = temp_dir.path().join(chosen_file_name);

                    fs::copy(&chosen_file_path, &temp_path)?;

                    edit_file(&temp_path)?;

                    return Ok(());
                }
            },
            Err(_e) => {}
        }
    }
    Ok(())
}
