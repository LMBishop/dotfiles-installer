mod config;
mod install;
mod util;
mod cli;

use clap::Parser;
use colored::*;
use std::process;
use config::Config;
use util::*;

fn main() {
    let args = cli::Args::parse();
    let install_profile = &args.file;
    let dry_run = &args.dry_run;

    let loaded_config: Config = Config::from_file(install_profile).unwrap_or_else(|err| {
        eprintln!("Cannot load '{install_profile}': {}", err.to_string());
        process::exit(EXIT_IO_ERROR);
    });
    
    let num_stages = loaded_config.stages.len();
    for i in 1..=num_stages {
        let stage = loaded_config.stages.get(i-1).unwrap();
        let name = stage.name.clone().unwrap();
        let count = format!("[{i}/{num_stages}]").yellow();

        println!("{} {}", count.bold(), name.bold());
        for step in stage.steps.as_ref().unwrap() {
            if *dry_run {
                println!("{}", fmt_step(step, &Ok(false)));
                continue;
            }

            let step_result = install::run_step(&step, &stage.base_path);

            println!("{}", fmt_step(step, &step_result));
            if step_result.is_err() {
                eprintln!("Step failed: {}", step_result.unwrap_err().to_string());
                println!();
                println!("{} {} {} {}{}", "Install stopped at stage".red(), i.to_string().red(), "of".red(), num_stages.to_string().red(), ".".red());
                process::exit(EXIT_INSTALL_FAILED);
            }
        }
        
        println!()
    }
    
    println!("{} {} {}", "Install of".bright_green(), install_profile.bright_green(), "completed.".bright_green())     
}

