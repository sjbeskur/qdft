
use tftp_server::server::{TftpServerBuilder};
use std::error::Error;
use std::net::{SocketAddr}; // IpAddr, Ipv4Addr, 

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn start_server(endpoint: SocketAddr) -> AppResult<()>{
    let mut server = TftpServerBuilder::new()
        .addr(endpoint)
        .serve_dir(std::path::PathBuf::from("."))
        .build().unwrap();

    println!("Server is listening on {:?}", endpoint);
    server.run();
    Ok(())
}
