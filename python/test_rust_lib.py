import rnbt as rnbt # This should match the name of your .so or .pyd file.

#logging.basicConfig(level=logging.INFO)
mc_binary = rnbt.load_binary('tests/resources/bigtest.nbt')
#mc_binary = rnbt.load_binary('tests/outputs/output_bt_py.json')
#err = mc_binary.to_json('tests/outputs/output_bt_py.json')
mc_version = mc_binary.get_mc_version()

for tag in mc_binary.tag_compounds_list:
    print(tag)
    