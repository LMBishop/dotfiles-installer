use crate::config::{CopyPath, Link, Step};
use std::{fs, path::PathBuf};
use crate::util::expand_home;

fn resolve_paths_and_mkdir(from: &String, to: &String, base_path: &PathBuf) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let expanded_home = &expand_home(&to);
    let destination = PathBuf::from(expanded_home);
    let mut source = base_path.clone();
    source.push(&from);
    
    let dest_parent = destination.parent().unwrap();
    fs::create_dir_all(dest_parent)?;
    Ok((source, destination))
}

fn ln(paths: &Link, base_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let (source, destination) = resolve_paths_and_mkdir(&paths.from, &paths.to, base_path)?;
    let source = source.as_path();
    let destination = destination.as_path();
    
    let _ = fs::remove_file(destination);
    fs::hard_link(source, destination)?;
    Ok(true)
}

fn ln_sym(paths: &Link, base_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let (source, destination) = resolve_paths_and_mkdir(&paths.from, &paths.to, base_path)?;
    let source = source.as_path();
    let destination = destination.as_path();
    
    let _ = fs::remove_file(destination);
    fs::soft_link(source, destination)?;
    Ok(true)
}

fn cp(paths: &CopyPath, base_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let (source, destination) = resolve_paths_and_mkdir(&paths.from, &paths.to, base_path)?;
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
        Step::Link(path) => { 
            match path {
                Link { symbolic: Some(true), .. } => ln_sym(path, base_path),
                _ => ln(path, base_path),
            }
        },
        Step::Copy(path) => { cp(path, base_path) },
        Step::Shell(command) => run_shell(command),
    }
}