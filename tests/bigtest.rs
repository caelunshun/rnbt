//! Tests the library using the `bigtest.nbt` file provided
//! by Mojang.
use flate2::read::GzDecoder;
use rnbt::{McWorldDescriptor,NbtTagCompound, NbtTagInt};
use std::io::prelude::*;
use std::path::PathBuf;



#[test]
fn bigtest() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/bigtest.nbt");

    let mut mc_world = McWorldDescriptor::new(path.to_str().unwrap());


    // let uncompressed_buf = std::fs::read(&path).unwrap();
    // let mut decoder = GzDecoder::new(uncompressed_buf.as_slice());

    // let mut buf = vec![];
    // let mut temp = [0u8; 16];
    // while let Ok(amnt) = decoder.read(&mut temp) {
    //     if amnt == 0 {
    //         break;
    //     }

    //     buf.extend_from_slice(&temp[..amnt]);
    // }

    // Parse NBT
    //let root = rnbt::parse_bytes(&buf).unwrap();

    // Confirm that values are correct
    let c = mc_world.unwrap().raw_data;

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
