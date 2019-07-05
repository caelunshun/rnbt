//! Tests the library using the `bigtest.nbt` file provided
//! by Mojang.
use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::path::PathBuf;

#[test]
fn bigtest() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/bigtest.nbt");

    let uncompressed_buf = std::fs::read(&path).unwrap();
    let mut decoder = GzDecoder::new(uncompressed_buf.as_slice());

    let mut buf = vec![];
    let mut temp = [0u8; 16];
    while let Ok(amnt) = decoder.read(&mut temp) {
        if amnt == 0 {
            break;
        }

        buf.extend_from_slice(&temp[..amnt]);
    }

    // Parse NBT
    let root = rnbt::parse(&buf).unwrap();
}
