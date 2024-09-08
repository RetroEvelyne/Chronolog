use fzf_wrapped::Fzf;
use std::error::Error;
use clap::ArgMatches;
use walkdir::WalkDir;

pub fn read_log(_args: &ArgMatches, logs_dir: &str) -> Result<(), Box<dyn Error>> {
    let file_names = get_all_files(&logs_dir)?;

    let chosen_file_name = choose_fuzzy(&file_names)?;

    println!("{chosen_file_name}");

    Ok(())
}

fn get_all_files(logs_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_names: Vec<String> = Vec::new();

    for file in WalkDir::new(logs_dir) {
        let file_string = file?.file_name()
            .to_str().expect("Failed to convert filename to string").to_string();
        if file_string.ends_with(".md") {
            file_names.push(file_string)
        }
    }

    Ok(file_names)
}

fn choose_fuzzy(file_names: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut fzf = Fzf::default();
    fzf.run()?;
    fzf.add_items(file_names)?;
    let chosen_file_name = fzf.output().expect("Failed to get fzf output");
    Ok(chosen_file_name)
}
