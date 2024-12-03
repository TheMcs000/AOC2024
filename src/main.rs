mod solution02;
use color_eyre::Result;


fn main() -> Result<()> {
    color_eyre::install()?;

    let res = solution02::solve();

    match res {
        Ok(()) => {Ok(())}
        Err(e) => {eprintln!("{}", e); Err(e)}
    }
}
