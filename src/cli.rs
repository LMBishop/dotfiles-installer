use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long, default_value_t = true)]
    pub dry_run: bool,
}