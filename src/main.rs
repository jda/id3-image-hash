extern crate getopts;
use std::env;
use std::path::Path;
use std::process;

use getopts::Options;
use sha1::{Sha1, Digest};

fn get_id3_image_hash(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let tag = match id3::Tag::read_from_path(file_path) {
        Ok(tag) => tag,
        Err(e) => return Err(Box::new(e)),
    };

    let first_pic = match tag.pictures().next() {
        Some(first_pic) => first_pic,
        None => return Ok(String::new()),
    };

    let mut hasher = Sha1::new();
    hasher.update(&first_pic.data);
    let hash: String = format!("{:x}", hasher.finalize());

    return Ok(hash);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "print file path if image in id3 tag matches HASH", "HASH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let hash_to_match = matches.opt_str("o");
    let mp3_file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mp3_file_path = Path::new(&mp3_file_name);

    let image_hash = match get_id3_image_hash(&mp3_file_path) {
        Ok(m) => { m }
        Err(f) => { 
            eprintln!("{}", f);
            process::exit(1);
        }
    };

    if image_hash.eq("") {
        eprintln!("no id3 tag in {}", mp3_file_name);
        return;
    };

    match hash_to_match {
        None => {
            println!("{}", image_hash);
            return;
        }
        Some(hash) => {
            if hash.eq(&image_hash) {
                println!("{}", mp3_file_name);
                return;
            }
        }
    }
}