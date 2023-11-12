//! Tests the library using the `bigtest.nbt` file provided
//! by Mojang.
use rnbt::McWorldDescriptor;
use std::path::PathBuf;

#[test]
fn read_region_file() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/r.0.0.mca");

    let mc_world = McWorldDescriptor::new(path);

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