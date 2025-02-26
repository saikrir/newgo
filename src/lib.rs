use std::{
    env, fs,
    io::{self, Error, ErrorKind, Write},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

const GO_CMD: &str = "go";
const META_FILE: &str = "newgo.json";
const MAIN_GO_CONTENT: &str = r#"
package main

import "fmt"

func main() {
	fmt.Println("Hello World")
}
"#;

#[derive(Debug)]
pub struct ProjectMetaData {
    workspace_dir: String,
    project_name: String,
    module_prefix: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct MetaInfo {
    workspace_dir: String,
    module_prefix: String,
}

pub fn print_banner() {
    println!("Welcome to Go Project Creator");
}

pub fn create_project(project_meta_data: ProjectMetaData) -> Result<(), Error> {
    let project_path =
        Path::new(&project_meta_data.workspace_dir).join(&project_meta_data.project_name);

    fs::create_dir(&project_path)?;

    env::set_current_dir(&project_path)?;

    let module_name = format!(
        "{}/{}",
        &project_meta_data.module_prefix, &project_meta_data.project_name
    );

    exec_cmd(&GO_CMD, &["mod", "init", &module_name])?;

    let main_go_path = PathBuf::from(&project_path).join("main.go");

    copy_file_to_path(&main_go_path)?;

    exec_cmd("code", &[&project_path.as_os_str().to_str().unwrap()])?;
    Ok(())
}

fn get_input<T, U>(field_name: U, message: U, field_validator: T) -> String
where
    T: Fn(&str) -> bool,
    U: AsRef<str> + std::fmt::Debug,
{
    let mut user_input: String = String::new();

    loop {
        print!("{message:?}: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                let user_input_trimmed = user_input.trim();
                if !field_validator(user_input_trimmed) {
                    println!("{field_name:?} is not valid, try again!");
                    user_input.clear();
                    continue;
                }
                return user_input_trimmed.to_string();
            }
            Err(_) => {
                println!("Error reading input for field {field_name:?}, please try again!");
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

    let prj_meta_data = get_project_metadata_from_file();

    let yes_no_validator =
        |f: &str| f.len() == 1 && f.to_uppercase() == "Y" || f.to_uppercase() == "N";

    let ws_field_name = "Workspace Directory";
    let use_default_ws_dir = get_input(
        ws_field_name,
        &format!(
            "use {} for workspace directory (Y/N): ",
            &prj_meta_data.workspace_dir
        ),
        yes_no_validator,
    );

    let workspace_dir = if use_default_ws_dir.to_uppercase() == "Y" {
        prj_meta_data.workspace_dir.clone()
    } else {
        get_input(
            ws_field_name,
            &format!("Please enter: {}", ws_field_name),
            ws_validator,
        )
    };

    let module_prefix_field = "Module Prefix";
    let use_default_module_prefix = get_input(
        module_prefix_field,
        &format!(
            "use {} for module prefix (Y/N): ",
            &prj_meta_data.module_prefix
        ),
        yes_no_validator,
    );

    let module_prefix = if use_default_module_prefix.to_uppercase() == "Y" {
        prj_meta_data.module_prefix.clone()
    } else {
        get_input(
            module_prefix_field,
            &format!("Please enter {}", module_prefix_field),
            &no_space_validator,
        )
    };

    let project_name_field = "Project Name";
    let project_name = get_input(
        project_name_field,
        &format!("Please enter {}", project_name_field),
        |prj_name| {
            let comps = prj_name.split_whitespace().collect::<Vec<&str>>();
            if comps.len() > 1 {
                return false;
            }
            if Path::new(&workspace_dir).join(prj_name).exists() {
                println!("{prj_name} exists in {workspace_dir}");
                return false;
            }

            return true;
        },
    );

    ProjectMetaData {
        module_prefix,
        workspace_dir,
        project_name,
    }
}

pub fn exec_cmd(program: &str, args: &[&str]) -> Result<(), Error> {
    let result = Command::new(program).args(args).output();

    match result {
        Ok(_ch) => {
            //println!("Process exited {:?} ", ch.wait_with_output()?.stdout);
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

pub fn check_defaults() {
    let meta_file = get_home_dir().join(META_FILE);
    if !meta_file.exists() {
        println!("lets setup some defaults!");
        io::stdout().flush().unwrap();
        create_project_metadata();
    }
}

fn get_project_metadata_from_file() -> MetaInfo {
    let meta_file = get_home_dir().join(META_FILE);
    if !meta_file.exists() {
        panic!("meta file does not exist");
    }

    let json_content = fs::read_to_string(meta_file).expect("error reading metafile");
    let meta_info = serde_json::from_str(&json_content).expect("failed to deserialize json");
    return meta_info;
}

fn create_project_metadata() -> MetaInfo {
    let meta_file = get_home_dir().join(META_FILE);
    let workspace_dir = get_input(
        "Workspace Directory",
        "Please enter Workspace Directory",
        |path| Path::new(path).exists(),
    );
    let module_prefix = get_input("Module Prefix", "Please enter module prefix", |pref| {
        pref.starts_with("github.com/")
    });

    let meta_info = MetaInfo {
        workspace_dir,
        module_prefix,
    };

    let json_data = serde_json::to_string_pretty(&meta_info).expect("error converting to json");
    fs::write(meta_file, json_data).expect("error writing to json");
    meta_info
}

fn get_home_dir() -> PathBuf {
    let home_dir = match env::home_dir() {
        Some(home_dir) => home_dir,
        None => PathBuf::from_str(".").unwrap(),
    };
    return home_dir;
}

pub fn detect_go_version() -> Result<(), Error> {
    exec_cmd("go", &vec!["version"])?;
    Ok(())
}

fn copy_file_to_path(target_path: &Path) -> Result<(), Error> {
    fs::write(target_path, MAIN_GO_CONTENT)?;
    Ok(())
}

// Print Greeting Message
// Go Installed ?
// Go Version ?
// Go Project Diretory
// New Project Name ?
