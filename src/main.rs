use std::io::{self, Write};

use clap::{Parser, Subcommand};

use makepdf::*;

// definition of command line args
#[derive(Parser)]
#[clap(author, about, version)]
/// make pdf from URL or images interactively.
struct Arg {}

// definition of interactive commands
#[derive(Parser)]
#[clap(name = "", disable_help_flag = true)]
/// make pdf from URL or images interactively.
struct OprArg {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// exit command
    Quit,

    /// backup files
    Backup,

    /// trim pictures
    Trim,

    /// generate pdf from pictures
    Genpdf,

    /// get image from URL
    Get { url: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse command line args (for help and version flags)
    Arg::parse();

    // show help message of commands
    // 1st element of argument is program name, so empty string
    if let Err(e) = OprArg::try_parse_from(["", "help"].into_iter()) {
        e.print()?;
        println!();
    }

    loop {
        {
            let mut stdout = io::stdout().lock();
            stdout.write_all(b"> ")?;
            stdout.flush()?;
        }

        let mut line = String::new();
        // return 0 if EOF
        if io::stdin().read_line(&mut line)? == 0 {
            println!();
            break;
        }

        // parse commands
        // 1st element of argument must be program name, so add empty string
        let cmds = [""].into_iter().chain(line.split_whitespace());
        let args = match OprArg::try_parse_from(cmds) {
            Ok(v) => v,
            Err(e) => {
                e.print()?;
                println!();
                continue;
            }
        };

        match args.cmd {
            Commands::Quit => {
                break;
            }
            Commands::Backup => {
                backup().await?;
            }
            Commands::Trim => {
                trim().await?;
            }
            Commands::Genpdf => {
                genpdf().await?;
            }
            Commands::Get { url } => {
                getimgs(&url).await?;
            }
        }
    }

    Ok(())
}
