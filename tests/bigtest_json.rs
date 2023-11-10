//! Tests the library using the `bigtest.nbt` file provided
//! by Mojang.
use rnbt::McWorldDescriptor;
use rnbt::nbt_tag;
use std::path::PathBuf;

#[test]
fn bigtest_json() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/bigtest.nbt");

    let mc_world = McWorldDescriptor::new(path);

    // Confirm that values are correct
    let c = mc_world.unwrap().raw_data;

    //write the contents to a json file
    c.to_json("tests/outputs/output_bt.json").unwrap();

    //read the content from a json file and populate the NbtTagCompound
    let c_json = nbt_tag::NbtTagCompound::from_json("tests/outputs/output_bt.json").unwrap();

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
