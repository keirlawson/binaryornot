use binaryornot;

#[test]
fn css_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/bootstrap-glyphicons.css").unwrap(), false);
}

#[test]
fn json_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/cookiecutter.json").unwrap(), false);
}

#[test]
fn perl2_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/perl_script").unwrap(), false);
}

#[test]
fn js_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/index.js").unwrap(), false);
}

#[test]
fn lua_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/no.lua").unwrap(), false);
}