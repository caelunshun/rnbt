use rnbt::McWorldDescriptor;
use std::path::PathBuf;



#[test]
fn litematica() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/test.litematic");

    let mc_world = McWorldDescriptor::new(path);

    // Confirm that values are correct
    let c = mc_world.unwrap().raw_data;

    let val_region = c.get("Regions").unwrap().compound().unwrap();
    let val_test = val_region.get("test").unwrap().compound().unwrap();
    let val_list = val_test.get("BlockStatePalette").unwrap().list().unwrap();
    // assert_eq!(c.get("BlockStatePalette").unwrap().int().unwrap().value, 2147483647);
    
}
