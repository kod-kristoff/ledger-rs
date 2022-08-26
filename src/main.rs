mod args;

use args::{Args, Command};
use clap::Parser;
use rledger::GlobalScope;
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();
    log::info!("rLedger starting");

    let global_scope = GlobalScope::new();

    let args = Args::parse();
    log::debug!("args = {:?}", &args);

    // let args = global_scope.read_command_argument(args);

    let mut status = 1;
    match args.command {
        Command::Accounts { reports_query } => {
            log::trace!("User has invoked a verb at the interactive command-line");
            // status = global_scope.execute_command_wrapper(args);
        }
    }
}
