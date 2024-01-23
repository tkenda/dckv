use criterion::{criterion_group, criterion_main, Criterion};
use std::io::Cursor;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use dckv_database::{RocksDB, RocksDBOpts};
use dckv_parser::Parser;

fn criterion_benchmark(c: &mut Criterion) {
    let config = RocksDBOpts::builder().path("rocksdb").build();
    let db = RocksDB::new(config);

    c.bench_function("parse stream", |b| {
        b.iter(|| async {
            let reader = File::open("test.dcm").await.unwrap();

            db.store(reader).await.unwrap();
        });
    });

    c.bench_function("parse cursor", |b| {
        b.iter(|| async {
            let mut file2 = File::open("test.dcm").await.unwrap();
            let mut contents = vec![];
            file2.read_to_end(&mut contents).await.unwrap();
            let reader = Cursor::new(contents);

            db.store(reader).await.unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
