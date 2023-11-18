//! Tests the library using one of my region files
//! by Mojang.
use flate2::read::GzDecoder;
use rnbt::{NbtTagCompound, NbtTagInt};
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufWriter;

#[test]
fn regions() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/20069.litematic");

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
    let root = rnbt::parse_bytes(&buf).unwrap();

    // Confirm that values are correct
    let c = root.compound().unwrap();

    let json_representation : String = root.to_json();
    //print!("root_json: {}", root_json);

    // Open a file for writing.
    let file = File::create("output.json").expect("Unable to create file");
    let writer = BufWriter::new(file);  // Using a BufWriter for more efficient writes.

    // Write the pretty-printed JSON to the file.
    serde_json::to_writer_pretty(writer, &json_representation).expect("Failed to write to file");


    assert_eq!(c.get("intTest").unwrap().int().unwrap().value, 2147483647);
    assert_eq!(c.get("byteTest").unwrap().byte().unwrap().value, 127);
    assert_eq!(
        c.get("doubleTest").unwrap().double().unwrap().value,
        0.49312871321823148
    );
    assert_eq!(
        c.get("floatTest").unwrap().float().unwrap().value,
        0.49823147058486938
    );
    assert_eq!(
        c.get("longTest").unwrap().long().unwrap().value,
        9223372036854775807
    );
    assert_eq!(c.get("shortTest").unwrap().short().unwrap().value, 32767);
}
