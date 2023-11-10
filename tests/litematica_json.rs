use rnbt::McWorldDescriptor;
use rnbt::nbt_tag;
use std::path::PathBuf;



#[test]
fn litematica_json() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/test.litematic");

    let mc_world = McWorldDescriptor::new(path);

    // Confirm that values are correct
    let c = mc_world.unwrap().raw_data;

    c.to_json("tests/outputs/litematica.json").unwrap();
    
    //read the content from a json file and populate the NbtTagCompound
    let c_json = nbt_tag::NbtTagCompound::from_json("tests/outputs/litematica.json").unwrap();

}
