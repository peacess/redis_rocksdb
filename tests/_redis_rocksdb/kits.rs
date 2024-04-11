use std::{fs, path};

use rocksdb::TransactionDB;

pub fn open_transaction_db(file: &str, name: &str) -> TransactionDB {
	let file_name = format!("temp/{}/{}.db", file, name);
	let db_path = path::Path::new(&file_name);
	if !db_path.exists() {
		fs::create_dir_all(db_path).expect("");
	}
	TransactionDB::open_default(db_path).expect("")
}

pub fn open_rocks_db(file: &str, name: &str) -> rocksdb::DB {
	let file_name = format!("temp/{}/{}.db", file, name);
	let db_path = path::Path::new(&file_name);
	if !db_path.exists() {
		fs::create_dir_all(db_path).expect("");
	}
	rocksdb::DB::open_default(db_path).expect("")
}
