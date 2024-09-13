pub fn save_ast_to_file(ast: &AST, file_path: &str) -> Result<(), None> {
    let encoded: Vec<u8> = bincode::serialize(ast).unwrap();
    let mut file = File::create(file_path)?;
    file.write_all(&encoded)?;
    Ok(())
}

pub fn load_ast_from_file(file_path: &str) -> Option<AST> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded: AST = bincode::deserialize(&buffer).unwrap();
    Some(decoded)
}

pub fn get_hash(file_path: &str) -> Option<String> {
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
    Some(hex::encode(&result[..16])) // First 16 bytes of the hash
}

pub fn file_exists_in_cache(file_path: &str) -> bool {
    let hash = match get_hash(file_path) {
        Some(hash) => hash,
        None => return false, // If hash calculation fails, assume file does not exist
    };

    let cache_file_path = format!("{}.zxcache", hash);
    Path::new(&cache_file_path).exists()
}
