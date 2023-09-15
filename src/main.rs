use clap::Parser;
use colored::Colorize;
use args::{Commands, Cli};
use interface::Interface;
use run::execute;
use assemble::assemble;

/// Contains functionality for assembling a source file. 
mod assemble;
/// Errors thrown during runnig the program. 
mod errors;
/// Contains functionality for interacting with the user/host system. 
mod interface;
/// Cotains types for parsing the args at startup. 
mod args;
/// Contains functionality for running a program. 
mod run;


fn main() {
    let cli = Cli::parse();
    let int = interface::CliInterface();
    
    let res = match cli.command {
        Commands::Run(v) => execute(v, &int),
        Commands::Assemble(a) => assemble(a, &int),
    };

    match res {
        Ok(_) => int.log_msg(format!("{}", "End. ".green())),
        Err(e) => {
            int.log_error("Error".to_owned());
            int.log_warn(e.describe());
        },
    }
    
}
