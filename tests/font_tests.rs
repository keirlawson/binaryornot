use binaryornot;

#[test]
fn eot_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/glyphiconshalflings-regular.eot").unwrap(), true);
}

#[test]
fn otf_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/glyphiconshalflings-regular.otf").unwrap(), true);
}

#[test]
fn ttf_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/glyphiconshalflings-regular.ttf").unwrap(), true);
}

#[test]
fn woff_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/glyphiconshalflings-regular.woff").unwrap(), true);
}