use dckv_database::{RocksDB, RocksDBConfig};

use dckv_parser::Parser;
use tokio::{fs::File, io::BufReader, join};

/*
async fn test_db() {
    // Open RocksDB database in main thread.
    let config = RocksDBConfig::builder().path("rocksdb").build();
    let db = RocksDB::open(&config).unwrap();

    let db1 = db.clone();
    let j1 = tokio::spawn(async move {
        for i in 0..10000 {
            println!("A");
            let key = format!("key{}", i).as_bytes().to_vec();
            let value = format!("value{}", i).as_bytes().to_vec();
            db1.put(&Category::Dataset, &key, &value).unwrap();
        }
    });

    let db2 = db.clone();
    let j2 = tokio::spawn(async move {
        for i in 0..10000 {
            println!("B");
            let key = format!("key{}", i).as_bytes().to_vec();
            let value = format!("value{}", i).as_bytes().to_vec();
            db2.put(&Category::Canvas, &key, &value).unwrap();
        }
    });

    let _ = join!(j1, j2);

    // get two key-value pars from dataset category
    let value = db.get(&Category::Dataset, b"key10").unwrap().unwrap();
    println!("1: {}", String::from_utf8(value).unwrap());

    let value = db.get(&Category::Dataset, b"key50").unwrap().unwrap();
    println!("2: {}", String::from_utf8(value).unwrap());

    // get two key-value pars from canvas category
    let value = db.get(&Category::Canvas, b"key100").unwrap().unwrap();
    println!("3: {}", String::from_utf8(value).unwrap());

    let value = db.get(&Category::Canvas, b"key500").unwrap().unwrap();
    println!("4: {}", String::from_utf8(value).unwrap());

    // get ref
    let value = db.get_ref(&Category::Dataset, b"key10").unwrap().unwrap();
    println!("5: {}", String::from_utf8(value.as_ref().to_vec()).unwrap());
}
*/

#[tokio::main]
async fn main() {
    //test_db().await;

    let config = RocksDBConfig::builder().path("rocksdb").build();
    let db = RocksDB::open(&config).unwrap();

    let file = File::open("test.dcm").await.unwrap();
    let reader = BufReader::new(file);

    db.store(reader).await.unwrap();
}
