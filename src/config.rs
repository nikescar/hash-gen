use clap::{Parser, Subcommand};

/// Command-line arguments parser structure.
///
/// It supports parsing for different hash algorithms like MD5, SHA-1, SHA-256,
/// and SHA-512.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The hash algorithm to use.
    ///
    /// This field accepts subcommands, allowing the user to specify the desired
    /// hashing algorithm (MD5, SHA-1, SHA-256, SHA-512).
    #[command[subcommand]]
    pub algorithm: Algorithm,
}

/// Enum representing different hashing algorithms.
///
/// Each variant corresponds to a specific hash algorithm and its associated
/// arguments, such as the file path for which the hash will be computed.
#[derive(Subcommand, Debug)]
pub enum Algorithm {
    /// Generate an MD5 hash
    #[command(
        about = "Generates an MD5 hash of the file or buffer",
        long_about = "Computes the MD5 hash of the given file or buffer, providing a 128-bit hash value."
    )]
    Md5 {
        /// Path to the file for which the MD5 hash will be generated.
        #[arg(short, long, conflicts_with = "buffer")]
        path: Option<std::path::PathBuf>,

        /// Buffer for which the MD5 hash will be generated.
        #[arg(short, long, conflicts_with = "path")]
        buffer: Option<String>,
    },

    /// Generate a SHA-1 hash
    #[command(
        about = "Generates a SHA-1 hash of the file or buffer",
        long_about = "Computes the SHA-1 hash of the given file or buffer, providing a 160-bit hash value."
    )]
    Sha1 {
        /// Path to the file for which the SHA-1 hash will be generated.
        #[arg(short, long, conflicts_with = "buffer")]
        path: Option<std::path::PathBuf>,

        /// Buffer for which the SHA-1 hash will be generated.
        #[arg(short, long, conflicts_with = "path")]
        buffer: Option<String>,
    },

    /// Generate a SHA-256 hash
    #[command(
        about = "Generates a SHA-256 hash of the file or buffer",
        long_about = "Computes the SHA-256 hash of the given file or buffer, providing a 256-bit hash value."
    )]
    Sha256 {
        /// Path to the file for which the SHA-256 hash will be generated.
        #[arg(short, long, conflicts_with = "buffer")]
        path: Option<std::path::PathBuf>,

        /// Buffer for which the SHA-256 hash will be generated.
        #[arg(short, long, conflicts_with = "path")]
        buffer: Option<String>,
    },

    /// Generate a SHA-512 hash
    #[command(
        about = "Generates a SHA-512 hash of the file or buffer",
        long_about = "Computes the SHA-512 hash of the given file or buffer, providing a 512-bit hash value."
    )]
    Sha512 {
        /// Path to the file for which the SHA-512 hash will be generated.
        #[arg(short, long, conflicts_with = "path")]
        path: Option<std::path::PathBuf>,

        /// Buffer for which the SHA-512 hash will be generated.
        #[arg(short, long, conflicts_with = "path")]
        buffer: Option<String>,
    },
}
