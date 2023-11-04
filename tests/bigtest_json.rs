//! Tests the library using the `bigtest.nbt` file provided
//! by Mojang.
use flate2::read::GzDecoder;
use rnbt::{NbtTagCompound, NbtTagInt};
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
    let root = rnbt::parse_bytes(&buf).unwrap();

    // Confirm that values are correct
    let c = root.compound().unwrap();

    //write the contents to a json file
    c.to_json("tests/outputs/output.json").unwrap();

    //read the content from a json file and populate the NbtTagCompound
    let c_json = NbtTagCompound::from_json("tests/outputs/output.json").unwrap();

    //assert the content of the new NbtTagCompound read from the json file
    assert_eq!(c_json.get("intTest").unwrap().int().unwrap().value, 2147483647);
    assert_eq!(c_json.get("byteTest").unwrap().byte().unwrap().value, 127);
    assert_eq!(
        c.get("doubleTest").unwrap().double().unwrap().value,
        0.49312871321823148
    );
    assert_eq!(
        c_json.get("floatTest").unwrap().float().unwrap().value,
        0.49823147058486938
    );
    assert_eq!(
        c_json.get("longTest").unwrap().long().unwrap().value,
        9223372036854775807
    );
    assert_eq!(c_json.get("shortTest").unwrap().short().unwrap().value, 32767);
}
