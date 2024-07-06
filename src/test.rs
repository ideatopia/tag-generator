use crate::util::*;

#[test]
fn test_read_words_from_file_basic() {
    // Define the path to the test.txt file within the dictionaries folder
    let filename = "dictionaries/test_read_words_from_file_basic.txt";

    // Create the test.txt file with known content
    std::fs::create_dir_all("dictionaries").expect("Failed to create dictionaries directory");
    std::fs::write(filename, "apple\nbanana\ncherry").expect("Failed to write test file");

    // Call the function under test
    let result = read_words_from_file(filename);

    // Assert the result
    assert!(result.is_ok());
    let words = result.unwrap();
    assert_eq!(words, vec!["apple", "banana", "cherry"]);

    // Clean up: delete the temporary file
    std::fs::remove_file(filename).expect("Failed to delete test file");
}

#[test]
fn test_read_words_from_file_empty() {
    // Define the path to the test.txt file within the dictionaries folder
    let filename = "dictionaries/test_read_words_from_file_empty.txt";

    // Create the test.txt file as empty
    std::fs::create_dir_all("dictionaries").expect("Failed to create dictionaries directory");
    std::fs::write(filename, "").expect("Failed to write test file");

    // Call the function under test
    let result = read_words_from_file(filename);

    // Assert the result
    assert!(result.is_ok());
    let words = result.unwrap();
    assert!(words.is_empty());

    // Clean up: delete the temporary file
    std::fs::remove_file(filename).expect("Failed to delete test file");
}

#[test]
fn test_read_words_from_file_not_found() {
    // Define a non-existent file path
    let filename = "dictionaries/non-existent.txt";

    // Call the function with the non-existent file path
    let result = read_words_from_file(filename);

    // Assert the result
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_read_words_from_file_multiple_lines() {
    // Define the path to the test.txt file within the dictionaries folder
    let filename = "dictionaries/test_read_words_from_file_multiple_lines.txt";

    // Create the test.txt file with multiple lines
    std::fs::create_dir_all("dictionaries").expect("Failed to create dictionaries directory");
    std::fs::write(filename, "apple\nbanana\ncherry\ndate").expect("Failed to write test file");

    // Call the function under test
    let result = read_words_from_file(filename);

    // Assert the result
    assert!(result.is_ok());
    let words = result.unwrap();
    assert_eq!(words, vec!["apple", "banana", "cherry", "date"]);

    // Clean up: delete the temporary file
    std::fs::remove_file(filename).expect("Failed to delete test file");
}

#[test]
fn test_read_words_from_windows_line_formatting() {
    // Define the path to the test.txt file within the dictionaries folder
    let filename = "dictionaries/test_read_words_from_windows_line_formatting.txt";

    // Create the test.txt file with Windows-style line endings
    std::fs::create_dir_all("dictionaries").expect("Failed to create dictionaries directory");
    std::fs::write(filename, "apple\r\nbanana\r\ncherry\r\n").expect("Failed to write test file");

    // Call the function under test
    let result = read_words_from_file(filename);

    // Assert the result
    assert!(result.is_ok());
    let words = result.unwrap();
    assert_eq!(words, vec!["apple", "banana", "cherry"]);

    // Clean up: delete the temporary file
    std::fs::remove_file(filename).expect("Failed to delete test file");
}
