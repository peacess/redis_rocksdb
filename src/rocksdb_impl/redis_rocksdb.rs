use anyhow::Result;
use ckb_rocksdb::{prelude::Open, DBCompressionType, Options, TransactionDB};
use std::{fs, path};

pub struct RedisRocksdb {
    pub(crate) db: ckb_rocksdb::TransactionDB,
}

impl RedisRocksdb {
    pub fn new(db: ckb_rocksdb::TransactionDB) -> Self {
        RedisRocksdb { db }
    }
}

pub fn open<Str: AsRef<str>>(fp: Str) -> Result<TransactionDB> {
    let db_path = path::Path::new(fp.as_ref());
    if !db_path.exists() {
        fs::create_dir_all(db_path)?;
    }

    let mut opt = Options::default();
    opt.create_if_missing(true);
    opt.create_missing_column_families(true);
    opt.set_compression_type(DBCompressionType::Lz4);
    opt.set_bottommost_compression_type(DBCompressionType::Zstd);
    //opt.set_enable_blob_files(true);

    Ok(ckb_rocksdb::TransactionDB::open(&opt, db_path)?)
}
