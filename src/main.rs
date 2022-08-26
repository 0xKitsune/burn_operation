mod burn_operation;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "Burn Operation",
    about = "Securely wipe a computer at the speed of light."
)]
struct Args {
    #[clap(
        short,
        long,
        help = "Path to file or directory to wipe. If no argument is included, the default path is `/` which will wipe the entire computer.",
        default_value = "/"
    )]
    pub path: PathBuf,

    #[clap(
        short,
        long,
        help = "Number of iterations when wiping each file.",
        default_value = "25"
    )]
    pub n: usize,

    #[clap(
        short,
        long,
        parse(from_flag),
        help = "Initializes a dead man's switch from a `dead_mans_switch.toml` file."
    )]
    pub dead_mans_switch: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.dead_mans_switch {
        //read in the dead man's switch toml file
    } else {
        //burn everything from starting from root
        burn_operation::burn::burn_system(args.path, &args.n)?;
    }

    Ok(())
}
