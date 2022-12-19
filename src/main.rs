use std::io::{self, Write};
use itertools::Itertools;

use makepdf::*;

enum Operation {
    Help,
    Quit,
    Backup,
    Trim,
    Genpdf,
    Get(String),
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    loop {
        {
            let mut stdout = io::stdout().lock();
            stdout.write_all(b"> ")?;
            stdout.flush()?;
        }

        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            println!("");
            break;
        }

        let mut cmds = line.split_whitespace();
        let cmd = cmds.next();
        let args = cmds.collect_vec();

//            match cmd {
//                "help"   => help(),
//                "quit"   => break,
//                "backup" => backup().await?,
//                "trim"   => trim().await?,
//                "genpdf" => genpdf().await?,
//                "get"    => {
//                    if args.len() > 0 {
//                        getimgs(args[0]).await?
//                    } else {
//                        println!("missing 1 argument for operation get")
//                    }
//                },
//                other => {
//                    println!("no such command: {}", other)
//                },
//            }
    }

    Ok(())
}

