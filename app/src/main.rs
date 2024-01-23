use dckv_database::{RocksDB, RocksDBOpts};

use dckv_parser::Parser;
use tokio::fs::File;
use tokio::io::BufReader;

#[tokio::main]
async fn main() {
    let opts = RocksDBOpts::builder().path("rocksdb").build();
    let db = RocksDB::new(opts);

    let file1 = File::open("test.dcm").await.unwrap();
    let reader1 = BufReader::new(file1);

    let db_ref = db.clone();
    let handle1 = tokio::spawn(async move {
        db_ref.store(reader1).await.unwrap();
    });

    let file2 = File::open("test.dcm").await.unwrap();
    let reader2 = BufReader::new(file2);

    let db_ref = db.clone();
    let handle2 = tokio::spawn(async move {
        db_ref.store(reader2).await.unwrap();
    });

    let _ = tokio::join!(handle1, handle2);
}
