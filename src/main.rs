use rledger::GlobalScope;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();
    log::info!("rLedger starting");

    let global_scope = GlobalScope::new();

    let args = std::env::args();
    log::debug!("args = {:?}", &args);

    let args = global_scope.read_command_argument(args);

    let mut status = 1;
    if !args.is_empty() {
        log::trace!("User has invoked a verb at the interactive command-line");
        status = global_scope.execute_command_wrapper(args);
    }
}
