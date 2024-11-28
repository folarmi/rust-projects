// use::clap::Parser;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "stringer - a simple CLI to transform and inspect strings", long_about = "stringer is a super fancy CLI (kidding)

One can use stringer to modify or inspect strings straight from the terminal")]

struct Cli {
    #[command(subcommand)]
    command:Option<Commands>,
}

#[derive(SubCommand)]
enum Commands {
    /// Reverses a string
    Reverse(Reverse),
     /// Inspects a string
    Inspect(Inspect),
}

#[derive(Args)]
struct Reverse {
    /// The string to reverse
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    /// The string to inspect
    string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}

fn main() {
    let cli = Cli::parse();

   
}
