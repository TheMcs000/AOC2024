mod solution03;
use color_eyre::Result;


fn main() -> Result<()> {
    color_eyre::install()?;

    let res = solution03::solve();

    match res {
        Ok(()) => {Ok(())}
        Err(e) => {eprintln!("{}", e); Err(e)}
    }
}
