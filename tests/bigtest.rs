//! Tests the library using the `bigtest.nbt` file provided
//! by Mojang.
use rnbt::McWorldDescriptor;
use std::path::PathBuf;

#[test]
fn bigtest() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/bigtest.nbt");

    let mc_world = McWorldDescriptor::new(path);

    // Confirm that values are correct
    let mc_world = mc_world.unwrap();
    let c = mc_world.tag_compounds_list.get(0).unwrap();

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
