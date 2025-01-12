use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, Error, ErrorKind, Write},
    path::Path,
    process::{Child, Command},
};

#[derive(Debug)]
pub struct ProjectMetaData {
    workspace_dir: String,
    project_name: String,
    module_prefix: String,
}

pub fn print_banner() {
    println!("Welcome to Go Project Creator");
}

pub fn create_project(project_meta_data: ProjectMetaData) -> Result<(), Error> {
    let go_cmd = "go";

    let project_path =
        Path::new(&project_meta_data.workspace_dir).join(&project_meta_data.project_name);

    fs::create_dir(&project_path)?;

    env::set_current_dir(&project_path)?;

    let module_name = format!(
        "{}/{}",
        &project_meta_data.module_prefix, &project_meta_data.project_name
    );

    exec_cmd(&go_cmd, &["mod", "init", &module_name])?;

    exec_cmd("code", &[&project_path.as_os_str().to_str().unwrap()])?;

    Ok(())
}

fn get_input<T>(field_name: &str, field_validator: T) -> String
where
    T: Fn(&str) -> bool,
{
    let mut user_input: String = String::new();

    loop {
        print!("Please enter {field_name} :");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                let user_input_trimmed = user_input.trim();
                if !field_validator(user_input_trimmed) {
                    println!("{field_name} is not valid, try again!");
                    user_input.clear();
                    continue;
                }
                return user_input_trimmed.to_string();
            }
            Err(_) => {
                println!("Error reading input, please try again!");
                continue;
            }
        };
    }
}

pub fn get_project_meta_data() -> ProjectMetaData {
    let ws_validator = |ws_dir: &str| {
        let path = Path::new(ws_dir);
        path.exists() && path.is_dir()
    };

    let no_space_validator = |field_value: &str| {
        let comps = field_value.split_whitespace().collect::<Vec<&str>>();
        comps.len() == 1
    };

    let workspace_dir = get_input("Workspace Directory", ws_validator);
    let project_name = get_input("Project Name", &no_space_validator);
    let module_prefix = get_input("Module Prefix", &no_space_validator);

    ProjectMetaData {
        module_prefix,
        workspace_dir,
        project_name,
    }
}

pub fn exec_cmd(program: &str, args: &[&str]) -> Result<(), Error> {
    let result = Command::new(program).args(args).spawn();

    match result {
        Ok(ch) => {
            println!("Process exited {:?} ", ch.id());
            Ok(())
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                println!("No file found '{}'", program)
            }
            Err(err)
        }
    }
}

// Print Greeting Message
// Go Installed ?
// Go Version ?
// Go Project Diretory
// New Project Name ?
