use crate::config::{Step, CopyPath};
use std::{fs, path::PathBuf};
use crate::util::expand_home;

fn resolve_paths_and_mkdir(paths: &CopyPath, base_path: &PathBuf) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let expanded_home = &expand_home(&paths.to);
    let destination = PathBuf::from(expanded_home);
    let mut source = base_path.clone();
    source.push(&paths.from);
    
    let dest_parent = destination.parent().unwrap();
    fs::create_dir_all(dest_parent)?;
    Ok((source, destination))
}

fn ln(paths: &CopyPath, base_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let (source, destination) = resolve_paths_and_mkdir(paths, base_path)?;
    let source = source.as_path();
    let destination = destination.as_path();
    
    let _ = fs::remove_file(destination);
    fs::hard_link(source, destination)?;
    Ok(true)
}

fn cp(paths: &CopyPath, base_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let (source, destination) = resolve_paths_and_mkdir(paths, base_path)?;
    let source = source.as_path();
    let destination = destination.as_path();

    if fs::metadata(destination).is_ok() {
        return Ok(false); 
    }
    fs::copy(source, destination)?;
    Ok(true)
}

fn run_shell(command: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let exit = std::process::Command::new("/bin/sh").arg("-c").arg(command).output()?;
    return Ok(exit.status.success());
}

pub fn run_step(step: &Step, base_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    match step {
        Step::Link(path) => { ln(path, base_path) },
        Step::Copy(path) => { cp(path, base_path) },
        Step::Shell(command) => run_shell(command),
    }
}