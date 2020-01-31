#![warn(rust_2018_idioms)]

use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_test::io::Builder;

#[tokio::test]
async fn read() {
    let mut mock = Builder::new().read(b"hello ").read(b"world!").build();

    let mut buf = [0; 256];

    let n = mock.read(&mut buf).await.expect("read 1");
    assert_eq!(&buf[..n], b"hello ");

    let n = mock.read(&mut buf).await.expect("read 2");
    assert_eq!(&buf[..n], b"world!");
}

#[tokio::test]
async fn write() {
    let mut mock = Builder::new().write(b"hello ").write(b"world!").build();

    mock.write_all(b"hello ").await.expect("write 1");
    mock.write_all(b"world!").await.expect("write 2");
}

#[tokio::test]
async fn wait() {
    let mut now = Instant::now();
    let mut buf = [0; 256];
    let mut read_mock = Builder::new()
        .wait(Duration::from_millis(500))
        .read(b"hello ")
        .wait(Duration::from_millis(500))
        .read(b"world! ")
        .build();
    read_mock.read(&mut buf).await.expect("read 1");
    read_mock.read(&mut buf).await.expect("read 2");
    assert_eq!(1, now.elapsed().as_secs());

    now = Instant::now();
    let mut write_mock = Builder::new().wait(Duration::from_secs(1)).write(b"hello ").write(b"world!").build();
    write_mock.write_all(b"hello ").await.expect("write 1");
    write_mock.write_all(b"world!").await.expect("write 2");
    assert_eq!(1, now.elapsed().as_secs());
}
