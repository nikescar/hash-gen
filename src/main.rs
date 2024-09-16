#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

mod config;
mod hasher;

use clap::Parser;

fn main() {
    let args = config::Args::parse();

    match args.algorithm {
        config::Algorithm::Md5 { path, buffer } => {
            process_hash(hasher::Md5Hash::new(), path, buffer, "MD5");
        }
        config::Algorithm::Sha1 { path, buffer } => {
            process_hash(hasher::Sha1Hash::new(), path, buffer, "SHA1");
        }
        config::Algorithm::Sha256 { path, buffer } => {
            process_hash(hasher::Sha256Hash::new(), path, buffer, "SHA256");
        }
        config::Algorithm::Sha512 { path, buffer } => {
            process_hash(hasher::Sha512Hash::new(), path, buffer, "SHA512");
        }
    }
}

fn process_hash<H: hasher::Hash>(
    mut hasher: H,
    path: Option<std::path::PathBuf>,
    buffer: Option<String>,
    algorithm_name: &str,
) {
    if let Some(path) = path {
        match hasher.generate_from_file(path.as_path()) {
            Ok(hash) => println!(
                "{} hash ({}) = {}",
                algorithm_name,
                path.to_str().unwrap_or_default(),
                hash
            ),
            Err(e) => eprintln!("Failed to generate {algorithm_name} hash! Error: {e}"),
        }
    } else if let Some(buffer) = buffer {
        println!(
            "{} hash = {}",
            algorithm_name,
            hasher.generate_from_buf(buffer.as_bytes())
        );
    } else {
        eprintln!("No input provided for {algorithm_name} hash generation!");
    }
}
