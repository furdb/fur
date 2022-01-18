use clap::Parser;

/// A low-key database management system
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Operation to perform
    #[clap(short, long)]
    op: String,

    /// Name of the database
    #[clap(short, long)]
    db: String,

    /// Name of the table
    #[clap(short, long)]
    table: String,
}
