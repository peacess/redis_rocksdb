pub fn new_db_key() -> Vec<u8> {
    xid::new().to_string().as_bytes().to_vec()
}