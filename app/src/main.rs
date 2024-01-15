use dckv_database::{RocksDB, RocksDBConfig};

use dckv_parser::Parser;
use tokio::fs::File;
use tokio::io::BufReader;

#[tokio::main]
async fn main() {
    let config = RocksDBConfig::builder().path("rocksdb").build();
    let db = RocksDB::open(&config).unwrap();

    let file1 = File::open("test.dcm").await.unwrap();
    let reader = BufReader::new(file1);

    db.store(reader).await.unwrap();
}
