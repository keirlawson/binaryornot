use binaryornot;

#[test]
fn pyc_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/hello_world.pyc").unwrap(), true);
}

#[test]
fn empty_pyc_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/empty.pyc").unwrap(), true);
}

#[test]
fn troublesome_pyc_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/troublesome.pyc").unwrap(), true);
}