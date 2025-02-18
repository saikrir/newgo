use std::io::Error;

use newgo::{create_project, get_project_meta_data, print_banner};

fn main() -> Result<(), Error> {
    print_banner();

    // let pm = get_project_meta_data();
    // println!("{:?}", pm);

    // create_project(pm)?;
    Ok(())
}
