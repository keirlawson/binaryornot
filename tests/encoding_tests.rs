use binaryornot;

#[test]
fn utf16_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/bom_utf-16.txt").unwrap(), false);
}

#[test]
fn utf16le_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/bom_utf-16le.txt").unwrap(), false);
}

#[test]
fn utf16be_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/test-utf16be.txt").unwrap(), false);
}

#[test]
fn utf82_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/utf_8.txt").unwrap(), false);
}

#[test]
fn gb2_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/test-gb2.txt").unwrap(), false);
}

#[test]
fn kr_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/test-kr.txt").unwrap(), false);
}

#[test]
fn latin_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/test-latin.txt").unwrap(), false);
}

#[test]
fn big5_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/big5.txt").unwrap(), false);
}

#[test]
fn gb_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/test-gb.txt").unwrap(), false);
}

#[test]
fn utf8_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/bom_utf-8.txt").unwrap(), false);
}

#[test]
fn big5b_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/big5_B.txt").unwrap(), false);
}

#[test]
fn shishi_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/test-shishi.txt").unwrap(), false);
}

#[test]
fn utfcn_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/encodings/utf8cn.txt").unwrap(), false);
}