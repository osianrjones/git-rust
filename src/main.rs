#[allow(unused_imports)]
use std::env;
use std::{fmt::format, hash, io::{BufWriter, Read, Write}, path::PathBuf};
#[allow(unused_imports)]
use std::fs;
use std::vec;
use clap::Error;
use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::Digest;

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
                let _ = cat_print(&blob.as_str());
            } else {
                println!("No -p flag selected.")
            }
        }
        //git hash-object -w /path/to/file
        "hash-object" => {
            if args[2] == "-w" {
                if !(args[3].is_empty()) {
                    hash_object(args[3])
                } else {
                    println!("No path to file entered.")
                }
            } else {
                println!("No -w flag selected.")
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

fn cat_print(blob: &str) -> Result<(), std::io::Error> {
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

fn hash_object(path: String) -> Result<(), std::io::Error> {
    //Open the path to the file
    let mut fileData = fs::read(&path)?;
    let mut buffer: Vec<u8> = Vec::new();

    //Create the blob for the hash
    let blob = format!("blob {}\0", fileData.len());

    //Create the hash
    let hash = {
        let mut hasher = sha1::Sha1::new();
        hasher.update(blob.as_bytes());
        hasher.update(fileData);
        hasher.finalize()
    };

    //Open or create .git path
    let mut path = PathBuf::from(".git/objects/");
    path.push(hex::encode(&hash[..1]));
    if !path.is_dir() {
        fs::create_dir(&path)?;
    }

    //Write the hash and encode it
    path.push(hex::encode(&hash[1..]));

    let output_file = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(&path);

    let mut zlib_encoder =
                ZlibEncoder::new(Vec::new(), Compression::default());
    
    zlib_encoder.write_all(blob.as_bytes());

    zlib_encoder.write_all(&fileData);

    println!("{}", hex::encode(hash));

    Ok(())
}
