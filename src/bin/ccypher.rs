use std::env::args;
use learn::{encrypt, decrypt};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();
    if args.len() == 4 {
        let fn_name = &args[1];
        let raw_text = &args[2];
        let key = str::parse::<usize>(&args[3]).unwrap();
        if fn_name == "en" {
            let d = encrypt(raw_text, key);
            println!("After encrypt {}->{}", raw_text, d);
        }
        if fn_name == "de" {
            let d = decrypt(raw_text, key);
            println!("After decrypt {}->{}", raw_text, d);
        }
    }
    Ok(())
}