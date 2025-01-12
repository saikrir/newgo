use std::io::Error;

use newgo::{exec_cmd, print_banner};

fn main() -> Result<(), Error> {
    print_banner();
    exec_cmd("go", &["version"])?;
    Ok(())
}
