use std::process::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::env;


fn main() {
    let mut currentPath:String = String::from("C:\\");
    if let Ok(path) = env::current_dir() {
        if let Some(path_str) = path.to_str() {
            currentPath = path_str.to_string();
        }
    }
    loop{
        let mut _command = String::new();
        print!("{}> ",currentPath.clone());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut _command).expect("error");
        _command = _command.to_lowercase();
        let command = _command.trim();
        if command == "exit"{
            return;
        }else if command == "cd"{
            println!("{}",currentPath);
        }else if command.starts_with("cd->"){
            let _path = command.split("cd->").collect::<Vec<&str>>()[1];
            let path = Path::new(_path);
            if path.exists() && path.is_dir() {
                currentPath = path.to_str().unwrap_or("").to_string();
            }
            else{
                let mut __path = currentPath.clone();
                __path.push_str("\\");
                __path.push_str(_path);
                if Path::new(&__path).exists() && Path::new(&__path).is_dir(){
                    currentPath = (*__path.clone()).to_string();
                }
            }
        } else if command == "ls" {
            getfolders(currentPath.as_str());
        }else if(command.starts_with("text::read->")){
            let _path = command.split("text::read->").collect::<Vec<&str>>()[1];
            let path = Path::new(_path);
            if path.exists() && path.is_file() {
                let data = fs::read_to_string(path).unwrap_or(String::new());
                println!("{}",data);
            }
        }else if(command.starts_with("hex::read->")){
            let _path = command.split("hex::read->").collect::<Vec<&str>>()[1];
            let path = Path::new(_path);
            if path.exists() && path.is_file() {
                let data = fs::read(path).unwrap_or(Vec::new());
                println!("{:X?}",data);
            }
        }else if(command.starts_with("mkdir->")){
            let _path = command.split("mkdir->").collect::<Vec<&str>>()[1];
            match  fs::create_dir(format!("{}/{}",currentPath,_path)){
                Err(e)=>println!("error {}",e),
                Ok(e)=>println!("complete"),
                _ => {}
            }
        }else if(command.starts_with("mkfile->")){
            let _path = command.split("mkfile->").collect::<Vec<&str>>()[1];
            match  fs::File::create(format!("{}/{}",currentPath,_path)){
                Err(e)=>println!("error {}",e),
                Ok(e)=>println!("complete"),
                _ => {}
            }
        }
        else{
            if command.contains(" "){
                let arguments = command.split(" ").collect::<Vec<&str>>();
                let processname = command.split_whitespace().next().unwrap_or("");
                runProcessWithArg(processname, arguments,currentPath.as_str())
            }else{
                runProcess(command,currentPath.as_str());
            }
        }
    }
}
fn runProcessWithArg(processName: &str, arguments: Vec<&str>, currentPath: &str) {
    let mut command_result = Command::new(processName);
    for (index, arg) in arguments.iter().enumerate() {
        if index > 0 {
            command_result.arg(arg);
        }
    }
    
    if let Ok(mut child) = command_result.spawn() {
        if let Ok(exit_status) = child.wait() {
            match exit_status.code() {
                Some(code) => {
                    println!("process exit with code {}", code)
                }
                None => {
                    println!("process terminated by signal")
                }
            }
        } else {
            println!("Failed to wait on child process");
        }
    } else {
        println!("Failed to execute command");
    }
}
fn runProcess(processName:&str,_currentPath:&str){
    let command_result = Command::new(processName)
    .current_dir(_currentPath)
    .status();

match command_result {
    Ok(exit_status) => {
        println!("process exit with code {}",exit_status.code().unwrap_or(0))
    }
    Err(e) => {
        println!("bad command bro: {}", e);
    }
}
}
fn getfolders(path:&str){
    let  folder = fs::read_dir(path).unwrap();
    for entry in folder {
        let entry = entry.unwrap();
        let path = entry.path();
        let thistype = fs::metadata(path.clone()).unwrap();
        if thistype.is_dir() {
            println!("folder: {} ", path.display());
        } else {
            println!("file: {} ", path.display());
        }
    }
}