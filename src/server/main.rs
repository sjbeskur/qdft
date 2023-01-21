
use async_tftp::server::TftpServerBuilder;
use async_tftp::Result;

fn main() -> Result<()> {

    let cfg = parse_args();

    println!("Starting Server at: {}", cfg.server_addr);
    let server_address = cfg.server_addr;
    smol::block_on(async { // or any other runtime/executor
        let tftpd = TftpServerBuilder::with_dir_wo(".")?
        .bind(server_address.parse().unwrap())
        .build().await?;


        tftpd.serve().await?;
        Ok(())
    })
}

pub struct Config{
    pub server_addr: String,
}

// total hack way to do this
pub fn parse_args() -> Config {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Invalid args");
        std::process::exit(1);
    }

    Config { 
        server_addr:  args[1].to_owned(),
    }
}