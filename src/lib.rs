use std::{io::Error, process::{Child, Command}};


pub fn print_banner() {
    println!("Welcome to Go Project Creator");
}


pub fn exec_cmd(program: &str, args: &[&str] ) -> Result<(), Error>{
    let result = Command::new(program)
    .args(args)
    .spawn();

    match result {
        Ok(ch) => {
            println!("Process exited {:?} ", ch.id());
            Ok(())
        }
        Err(err) => Err(err) 
    }
}

// Print Greeting Message
// Go Installed ?
// Go Version ?
// Go Project Diretory
// New Project Name ?
