#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about=None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// List all accounts for postings that match the report-query.
    Accounts {
        #[clap(value_parser)]
        reports_query: Option<String>,
    },
}
