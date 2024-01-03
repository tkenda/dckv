use dckv_database::{DBActions, DatabaseError, RocksDB, RocksDBConfig};

fn test_db() -> Result<(), DatabaseError> {
    // create rocksdb configuration
    let config = RocksDBConfig::builder().path("rocksdb").build();

    // create a new database instance at the given path and group (column family for RockDB) named uno
    let db1 = RocksDB::open(&config, "uno")?;

    // put two key-value pars
    db1.put(b"hola1", b"mundo1")?;
    db1.put(b"hola2", b"mundo2")?;

    drop(db1);

    // create a new database instance at the given path and group (column family for RockDB) named dos
    let db2 = RocksDB::open(&config, "dos")?;

    // put two key-value pars
    db2.put(b"hola3", b"mundo3")?;
    db2.put(b"hola4", b"mundo4")?;

    drop(db2);

    let db3 = RocksDB::open(&config, "uno")?;

    let value = db3.get(b"hola1")?.unwrap();
    println!("hola1: {}", String::from_utf8(value).unwrap());

    let value = db3.get(b"hola2")?.unwrap();
    println!("hola2: {}", String::from_utf8(value).unwrap());

    drop(db3);

    let db4 = RocksDB::open(&config, "dos")?;

    let value = db4.get(b"hola3")?.unwrap();
    println!("hola3: {}", String::from_utf8(value).unwrap());

    let value = db4.get(b"hola4")?.unwrap();
    println!("hola4: {}", String::from_utf8(value).unwrap());

    Ok(())
}

fn main() {
    println!("init");

    if let Err(err) = test_db() {
        println!("{}", err);
    }
}
