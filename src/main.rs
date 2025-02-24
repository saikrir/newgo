use std::io::Error;

use newgo::{
    check_defaults, create_project, detect_go_version, get_project_meta_data, print_banner,
};

fn main() -> Result<(), Error> {
    print_banner();
    if let Err(e) = detect_go_version() {
        println!("go not installed, wont continue further");
        return Err(e);
    }
    println!("go installation detected");

    check_defaults();

    let pm = get_project_meta_data();
    create_project(pm)?;
    Ok(())
}
