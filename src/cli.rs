pub struct Config{
    pub server_addr: String,
}

// total hack way to do this
pub fn parse_args() -> Config {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Invalid args.\n\tUsage: <ip_address>:<port>");
        std::process::exit(1);
    }

    Config { 
        server_addr:  args[1].to_owned(),
    }
}