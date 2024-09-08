use dirs::home_dir;
use std::error::Error;
use std::path::Path;
use std::fs;
use std::process::Command;

pub fn execute(path: &str, command: String) -> Result<(), Box<dyn Error>> {
    let _output = Command::new("git")
        .args(["-C", path])
        .args(command.split(' '))
        .output()?;

    Ok(())
}

pub fn logs_repo_string() -> String {
    let home_dir = home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();
    format!("{home_dir}/.local/share/chronolog")
}

pub fn ensure_logs_repo(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path).unwrap();

        execute(path, String::from("init"))?;
        execute(path, String::from("add --all"))?;
    }

    Ok(())
}
