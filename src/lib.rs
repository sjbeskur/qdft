

use std::error::Error;
use std::net::{SocketAddr}; // IpAddr, Ipv4Addr, 
use async_tftp::server::TftpServerBuilder;
use async_tftp::Result;

mod cli;
pub use cli::{ Config, parse_args };

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn run_server(config: Config) -> AppResult<()>{
    let server_address = config.server_addr;    
    println!("Starting Server at: {}", server_address);
    smol::block_on(async { // or any other runtime/executor
        let tftpd = TftpServerBuilder::with_dir_rw(".")?
        .bind(server_address.parse().unwrap())
        .build().await?;

        tftpd.serve().await?;
        Ok(())
    })
}
