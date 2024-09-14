#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

mod config;
mod hasher;

use clap::Parser;
use hasher::Hash;

// TODO: add vscode workspace with plugins

fn main() {
    let args = config::Args::parse();
    match args.algorithm {
        config::Algorithm::Md5 { path } => {
            let mut md5 = hasher::Md5Hash::new();
            match md5.generate_from_file(path.as_path()) {
                Ok(hash) => println!("MD5 ({}) = {}", path.to_str().unwrap_or_default(), hash),
                Err(e) => eprintln!("Failed to generate hash! Error: {e}"),
            }
        }
        config::Algorithm::Sha1 { path } => {
            let mut sha1 = hasher::Sha1Hash::new();
            match sha1.generate_from_file(path.as_path()) {
                Ok(hash) => println!("SHA1 ({}) = {}", path.to_str().unwrap_or_default(), hash),
                Err(e) => eprintln!("Failed to generate hash! Error: {e}"),
            }
        }
        config::Algorithm::Sha256 { path } => {
            let mut sha256 = hasher::Sha256Hash::new();
            match sha256.generate_from_file(path.as_path()) {
                Ok(hash) => println!("SHA256 ({}) = {}", path.to_str().unwrap_or_default(), hash),
                Err(e) => eprintln!("Failed to generate hash! Error: {e}"),
            }
        }
        config::Algorithm::Sha512 { path } => {
            let mut sha512 = hasher::Sha512Hash::new();
            match sha512.generate_from_file(path.as_path()) {
                Ok(hash) => println!("SHA512 ({}) = {}", path.to_str().unwrap_or_default(), hash),
                Err(e) => eprintln!("Failed to generate hash! Error: {e}"),
            }
        }
    }
}
