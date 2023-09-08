use clap::Parser;
use colored::Colorize;
use args::{Commands, Cli};
use run::execute;
use assemble::assemble;

mod assemble;
mod errors;
mod interface;
mod args;
mod run;


fn main() {
    let cli = Cli::parse();
    let int = interface::CliInterface();
    
    let res = match cli.command {
        Commands::Run(v) => execute(v, &int),
        Commands::Assemble(a) => assemble(a, &int),
    };

    match res {
        Ok(_) => println!("{}", "End. ".green()),
        Err(e) => {
            println!("{}", "Error. ".yellow());
            println!("{}", e.describe());
        },
    }
    
}
