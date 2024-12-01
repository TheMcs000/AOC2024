mod solution01;
use color_eyre::Result;


fn main() -> Result<()> {
    color_eyre::install()?;

    let res = solution01::solve();

    match res {
        Ok(()) => {Ok(())}
        Err(e) => {eprintln!("{}", e); Err(e)}
    }
}
