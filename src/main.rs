mod burn_operation;
use clap::Parser;
use std::env;
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[clap(
    name = "Burn Operation",
    about = "Securely wipe a computer at the speed of light."
)]
struct Args {
    #[clap(
        short,
        long,
        parse(from_flag),
        help = "Authorize burn_operation to run with root privledges. This flag only needs to be run once. If this flag is not used before running `burn_operation`, root access will need to be granted with sudo or equivalent."
    )]
    pub authorize: bool,

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
    if args.authorize {
        authorize()?;
    } else if args.dead_mans_switch {
        //read in the dead man's switch toml file
    } else {
        //burn everything from starting from root
        burn_operation::burn::burn_system(args.path, &args.n)?;
    }

    Ok(())
}

fn authorize() -> Result<(), Error> {
    let operating_system = env::consts::OS;
    let mut bin_path = home::cargo_home()?;
    bin_path.push("bin");

    match operating_system {
        "linux" => {
            Command::new("chmod")
                .current_dir(bin_path)
                .arg("666")
                .arg("burn_operation")
                .spawn()?;
        }
        "mac" => {}
        "windows" => {}

        _ => (),
    }

    Ok(())
}
