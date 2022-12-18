use std::io;

#[derive(Debug)]
enum Operation {
    Get,
    GenPdf
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        break;
    }

    Ok(())
}

