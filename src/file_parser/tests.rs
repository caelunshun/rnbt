#[cfg(test)]

use super::*;

#[test]
fn test_new_file_parser() {
    let file_path = PathBuf::from("path/to/file");
    let file_parser = FileParser::new(&file_path, ReadMode::EntireFile);
    assert_eq!(file_parser.file_path, file_path);
    assert_eq!(matches!(file_parser.read_mode, ReadMode::EntireFile), true);
}

#[test]
fn test_parse_function() {
    // Here you would create an instance of FileParser
    // and call the parse method, then assert the results.
    // This might require mocking the file I/O if it depends on actual files.
}

#[test]
fn test_read_entire_file() {
    // This test would require a mock file or a test file to read from.
    // You could create a FileParser instance and call read_entire_file
    // then assert that the contents are as expected.
}

#[test]
fn test_read_stream() {
    // As the read_stream method is not implemented yet,
    // this test should expect a panic or use the should_panic attribute.
    let file_parser = FileParser::new(&PathBuf::from("path/to/file"), ReadMode::Stream);
    assert!(file_parser.read_stream().is_err());
}

#[test]
fn test_parse_bytes() {
    // Provide a byte slice and test the parse_bytes function,
    // asserting the expected outcome.
    // Example:
    // let bytes = [/* byte data */];
    // let result = parse_bytes(&bytes);
    // assert!(result.is_ok());
    // assert_eq!(result.unwrap(), /* expected NbtTag value */);
}
