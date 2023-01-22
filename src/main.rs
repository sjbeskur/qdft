use qdfs::*;

fn main() -> AppResult<()> {
    let cfg = parse_args();
    run_server(cfg)?;
    Ok(())
}

