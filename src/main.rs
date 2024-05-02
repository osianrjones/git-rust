#[allow(unused_imports)]
use std::env;
use std::{fmt::format, io::Read};
#[allow(unused_imports)]
use std::fs;
use std::vec;

use clap::Error;
use flate2::bufread::ZlibDecoder;

fn main() {

    // Create default .git directory
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        //git init
        "init" => {
            let init_result = init();
            if init_result.is_err() {
                println!("Unable to create git directory.")
            }
        }
        //$ git cat-file -p <blob_sha>
        "cat-file" => {
            //Ensure -p flag selected
            if args[2] == "-p" {
                let blob: String = args[3].parse().unwrap();
                cat_print(&blob.as_str());
            } else {
                println!("No -p flag selected.")
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
    //Create .git dir
    let git_root_creation = fs::create_dir(".git")?;

    let init_dirs = vec![".git/objects", ".git/refs", ".git/HEAD"];
    for dirs in init_dirs {
        fs::create_dir(dirs)?;
    }

    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;

    println!("Initialized git directory.");

    Ok(())
}

fn cat_print(blob: &str) ->Result<(), std::io::Error> {
    //This is the path the blob should exist at
    let path = format!(".git/objects/{}/{}", &blob[0..2], &blob[2..]);

    //Read the blob using Zlib
    let blob_data = fs::read(&path).unwrap();
    let mut holdr = ZlibDecoder::new(&blob_data[..]);
    let mut decompressed = Vec::new();
    holdr.read_to_end(&mut decompressed).unwrap();

    //Split the null byte
     let parts: Vec<&[u8]> = decompressed.split(|&x| x == 0).collect();
     let content = parts[1];
     print!("{}", String::from_utf8(content.to_vec()).unwrap());

     Ok(())
}
