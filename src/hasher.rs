use digest::{Digest, FixedOutputReset};
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;
use sha2::Sha512;

/// A trait for calculating cryptographic hash values.
///
/// This trait defines a common interface for different hash algorithms, such as
/// MD5, SHA-1, SHA-256, and SHA-512. The trait requires an associated type
/// `Hasher` that must implement the `Digest` and `FixedOutputReset` traits.
///
/// It provides two main methods:
/// - `generate_from_buf`: Computes a hash from a given byte buffer.
/// - `generate_from_file`: Computes a hash from the contents of a file.
///
/// # Associated Types
///
/// - `Hasher`: The specific hash algorithm to be used, which must implement `Digest` and `FixedOutputReset`.
pub trait Hash {
    /// The specific hasher type that implements `Digest` and `FixedOutputReset`.
    type Hasher: Digest + FixedOutputReset;

    /// Provides a mutable reference to the hasher.
    ///
    /// This method is used internally by the trait's methods to access the hasher
    /// instance.
    fn hasher(&mut self) -> &mut Self::Hasher;

    /// Generates a hash from a byte buffer.
    ///
    /// This method takes a byte slice (`buf`) as input, updates the hasher with
    /// the contents of the buffer, and returns the resulting hash as a hexadecimal
    /// string.
    ///
    /// # Arguments
    ///
    /// - `buf`: A slice of bytes for which the hash should be computed.
    ///
    /// # Returns
    ///
    /// - A `String` containing the hexadecimal representation of the computed hash.
    fn generate_from_buf(&mut self, buf: &[u8]) -> String {
        let hasher = self.hasher();
        hasher.update(buf);
        hex::encode(hasher.finalize_reset())
    }

    /// Generates a hash from the contents of a file.
    ///
    /// This method reads the contents of the file located at the specified path,
    /// and computes its hash using the `generate_from_buf` method.
    ///
    /// # Arguments
    ///
    /// - `path`: A reference to a `std::path::Path` specifying the file to hash.
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: The computed hash as a hexadecimal string, if the file exists.
    /// - `Err(std::io::Error)`: An error if the file could not be found or read.
    fn generate_from_file(&mut self, path: &std::path::Path) -> Result<String, std::io::Error> {
        Ok(self.generate_from_buf(std::fs::read(path)?.as_slice()))
    }
}

/// A struct representing an MD5 hash calculator.
///
/// This struct wraps the `Md5` hasher and implements the `Hash` trait to provide
/// functionality for generating MD5 hashes.
pub struct Md5Hash {
    hasher: Md5,
}

impl Md5Hash {
    /// Creates a new `Md5Hash` instance.
    ///
    /// This function returns a new instance of `Md5Hash` with an initialized MD5 hasher.
    pub fn new() -> Self {
        Self { hasher: Md5::new() }
    }
}

impl Hash for Md5Hash {
    type Hasher = Md5;

    /// Returns a mutable reference to the internal MD5 hasher.
    fn hasher(&mut self) -> &mut Md5 {
        &mut self.hasher
    }
}

/// A struct representing a SHA-1 hash calculator.
///
/// This struct wraps the `Sha1` hasher and implements the `Hash` trait to provide
/// functionality for generating SHA-1 hashes.
pub struct Sha1Hash {
    hasher: Sha1,
}

impl Sha1Hash {
    /// Creates a new `Sha1Hash` instance.
    ///
    /// This function returns a new instance of `Sha1Hash` with an initialized SHA-1 hasher.
    pub fn new() -> Self {
        Self {
            hasher: Sha1::new(),
        }
    }
}

impl Hash for Sha1Hash {
    type Hasher = Sha1;

    /// Returns a mutable reference to the internal SHA-1 hasher.
    fn hasher(&mut self) -> &mut Sha1 {
        &mut self.hasher
    }
}

/// A struct representing a SHA-256 hash calculator.
///
/// This struct wraps the `Sha256` hasher and implements the `Hash` trait to provide
/// functionality for generating SHA-256 hashes.
pub struct Sha256Hash {
    hasher: Sha256,
}

impl Sha256Hash {
    /// Creates a new `Sha256Hash` instance.
    ///
    /// This function returns a new instance of `Sha256Hash` with an initialized SHA-256 hasher.
    pub fn new() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }
}

impl Hash for Sha256Hash {
    type Hasher = Sha256;

    /// Returns a mutable reference to the internal SHA-256 hasher.
    fn hasher(&mut self) -> &mut Sha256 {
        &mut self.hasher
    }
}

/// A struct representing a SHA-512 hash calculator.
///
/// This struct wraps the `Sha512` hasher and implements the `Hash` trait to provide
/// functionality for generating SHA-512 hashes.
pub struct Sha512Hash {
    hasher: Sha512,
}

impl Sha512Hash {
    /// Creates a new `Sha512Hash` instance.
    ///
    /// This function returns a new instance of `Sha512Hash` with an initialized SHA-512 hasher.
    pub fn new() -> Self {
        Self {
            hasher: Sha512::new(),
        }
    }
}

impl Hash for Sha512Hash {
    type Hasher = Sha512;

    /// Returns a mutable reference to the internal SHA-512 hasher.
    fn hasher(&mut self) -> &mut Sha512 {
        &mut self.hasher
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_md5_from_buf() {
        let mut md5 = Md5Hash::new();
        let hash_str = md5.generate_from_buf(b"Hello World!");
        assert_eq!(hash_str, "ed076287532e86365e841e92bfc50d8c");
    }

    #[test]
    fn generate_sha1_from_buf() {
        let mut sha1 = Sha1Hash::new();
        let hash_str = sha1.generate_from_buf(b"Hello World!");
        assert_eq!(hash_str, "2ef7bde608ce5404e97d5f042f95f89f1c232871");
    }

    #[test]
    fn generate_sha256_from_buf() {
        let mut sha256 = Sha256Hash::new();
        let hash_str = sha256.generate_from_buf(b"Hello World!");
        assert_eq!(
            hash_str,
            "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069"
        );
    }

    #[test]
    fn generate_sha512_from_buf() {
        let mut sha512 = Sha512Hash::new();
        let hash_str = sha512.generate_from_buf(b"Hello World!");
        assert_eq!(
            hash_str,
            "861844d6704e8573fec34d967e20bcfef3d424cf48be04e6dc08f2bd58c729743371015ead891cc3cf1c9d34b49264b510751b1ff9e537937bc46b5d6ff4ecc8"
        );
    }
}
