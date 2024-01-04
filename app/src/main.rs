use dckv_database::{Category, DBActions, DatabaseError, RocksDB, RocksDBConfig};

fn test_db() -> Result<(), DatabaseError> {
    // create rocksdb configuration
    let config = RocksDBConfig::builder().path("rocksdb").build();
    let db = RocksDB::open(&config)?;

    // put two key-value pars in dataset category
    db.put(Category::Dataset, b"hola1", b"mundo1")?;
    db.put(Category::Dataset, b"hola2", b"mundo2")?;

    // put two key-value pars in canvas category
    db.put(Category::Canvas, b"hola3", b"mundo3")?;
    db.put(Category::Canvas, b"hola4", b"mundo4")?;

    // get two key-value pars from dataset category
    let value = db.get(Category::Dataset, b"hola1")?.unwrap();
    println!("hola1: {}", String::from_utf8(value).unwrap());

    let value = db.get(Category::Dataset, b"hola2")?.unwrap();
    println!("hola2: {}", String::from_utf8(value).unwrap());

    // get two key-value pars from canvas category
    let value = db.get(Category::Canvas, b"hola3")?.unwrap();
    println!("hola3: {}", String::from_utf8(value).unwrap());

    let value = db.get(Category::Canvas, b"hola4")?.unwrap();
    println!("hola4: {}", String::from_utf8(value).unwrap());

    Ok(())
}

fn main() {
    println!("init");

    if let Err(err) = test_db() {
        println!("{}", err);
    }
}
