use binaryornot;

#[test]
fn empty_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/empty.txt").unwrap(), false);
}

#[test]
fn decoding_error_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/decoding-error").unwrap(), true);
}

#[test]
fn ds_store_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/.DS_Store").unwrap(), true);
}

#[test]
fn txt_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/robots.txt").unwrap(), false);
}

#[test]
fn unicode_txt_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/unicode.txt").unwrap(), false);
}

#[test]
fn russian_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/russian_file.rst").unwrap(), false);
}

#[test]
fn exe2_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/grep").unwrap(), true);
}