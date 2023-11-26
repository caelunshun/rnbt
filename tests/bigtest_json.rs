//! Tests the library using the `bigtest.nbt` file provided
//! by Mojang.
use rnbt::McWorldDescriptor;
use rnbt::nbt_tag;
use std::path::PathBuf;
use std::fs;
use std::io::BufReader;

#[test]
fn bigtest_json() {
    let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    
    let mut path_res = path.clone();
    let mut path_out = path.clone();
    path_res.push("tests/resources/bigtest.nbt");
    path_out.push("tests/outputs/output_bt.json");

    let mc_world = McWorldDescriptor::new(path_res);

    // Confirm that values are correct
    let mc_world = mc_world.unwrap();
    let c = mc_world.tag_compounds_list.get(0).unwrap().values.clone();

    mc_world.to_json(path_out.clone()).unwrap();

    // let file = fs::File::open(&path_out).unwrap();
    // let reader = BufReader::new(file); // Wrap the file in a BufReader, since very large file are expected.

    // // Deserialize the JSON data directly from the stream.;
    // let deserialized_nbt: String = serde_json::from_reader(reader).unwrap();
    let json_mc_world = McWorldDescriptor::new(path_out.clone()).unwrap();
    let c_json = json_mc_world.tag_compounds_list.get(0).unwrap().values.clone();

    //read the content from a json file and populate the NbtTagCompound
    //let c_json = nbt_tag::NbtTagCompound::from_json("tests/outputs/output_bt.json").unwrap();

    //assert the content of the new NbtTagCompound read from the json file
    // assert_eq!(c_json.get("intTest").unwrap().int().unwrap().value, 2147483647);
    // assert_eq!(c_json.get("byteTest").unwrap().byte().unwrap().value, 127);
    // assert_eq!(
    //     c.get("doubleTest").unwrap().double().unwrap().value,
    //     0.49312871321823148
    // );
    // assert_eq!(
    //     c_json.get("floatTest").unwrap().float().unwrap().value,
    //     0.49823147058486938
    // );
    // assert_eq!(
    //     c_json.get("longTest").unwrap().long().unwrap().value,
    //     9223372036854775807
    // );
    // assert_eq!(c_json.get("shortTest").unwrap().short().unwrap().value, 32767);
}
