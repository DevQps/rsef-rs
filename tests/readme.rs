#[cfg(feature = "download")]
#[tokio::test]
async fn readme() {
    use rsef_rs::{Line, Registry};

    // Friday 1 February 2019 21:22:48
    let timestamp = 1_549_056_168;
    let stream = Registry::download(Registry::RIPE, timestamp).await.unwrap();

    let records = rsef_rs::read_all(stream).unwrap();

    for x in records {
        match x {
            Line::Version(x) => println!("Version: {:?}", x),
            Line::Summary(x) => println!("Summary: {:?}", x),
            Line::Record(x) => println!("Record: {:?}", x),
        }
    }
}
