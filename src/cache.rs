use std::fs::File;
use crate::ast::AST;
use std::io::{Read, Write, BufReader};
use std::path::PathBuf;
use sha2::{Sha512, Digest};
use bincode;
use hex;

pub fn save_ast_to_file(ast: &AST, file_path: &str) -> Result<(), std::io::Error> {
    let encoded: Vec<u8> = bincode::serialize(ast)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
    let mut file = File::create(file_path)?;
    file.write_all(&encoded)?;
    Ok(())
}

pub fn load_ast_from_file(file_path: &str) -> Result<AST, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded: AST = bincode::deserialize(&buffer)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
    Ok(decoded)
}

pub fn get_hash(file_path: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha512::new();
    let mut buffer = [0; 8192]; // 8 KB buffer

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(&result[..16])) // First 16 bytes of the hash
}

pub fn file_exists_in_cache(file_path: &str, cache_dir: &str) -> bool {
    match get_hash(file_path) {
        Ok(hash) => {
            let cache_file_path = PathBuf::from(cache_dir).join(format!("{}.zxcache", hash));
            cache_file_path.exists()
        }
        Err(_) => false, // If hash calculation fails, assume that file does not exist
    }
}
