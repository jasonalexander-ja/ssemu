use clap::Parser;
use args::{Commands, Cli};
use run::execute;

mod assemble;
mod errors;
mod args;
mod run;


fn main() {
    let cli = Cli::parse();
    
    let res = match cli.command {
        Commands::Run(v) => execute(v),
        _ => Ok(())
    };
    
}
