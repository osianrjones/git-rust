#[allow(unused_imports)]
use std::env;
use std::fmt::format;
#[allow(unused_imports)]
use std::fs;
use std::vec;

fn main() {

    // Create default .git directory
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            let init_result = init();
            if init_result.is_err() {
                println!("Unable to create git directory.")
            }
        }
        _ => println!("Unknown command: {}", args[1])
    }
}

/*
    A function used to upon a "git init" call.
    Initializes all default directory required by git.
*/
#[allow(dead_code)]
#[allow(unused_variables)]
fn init() -> Result<(), std::io::Error>{
    //Create .git exists
    let git_root_creation = fs::create_dir(".git")?;

    let init_dirs = vec![".git/objects", ".git/refs", ".git/HEAD"];
    for dirs in init_dirs {
        fs::create_dir(dirs)?;
    }

    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;

    println!("Initialized git directory.");

    Ok(())
}
