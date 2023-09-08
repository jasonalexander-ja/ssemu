use clap::Parser;
use colored::Colorize;
use args::{Commands, Cli};
use interface::Interface;
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
        Commands::Assemble(a) => assemble(a),
    };

    match res {
        Ok(_) => int.log_msg(format!("{}", "End. ".green())),
        Err(e) => {
            int.log_error("Error".to_owned());
            int.log_warn(e.describe());
        },
    }
    
}
