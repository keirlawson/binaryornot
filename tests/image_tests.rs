use binaryornot;

#[test]
fn png_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/logo.png").unwrap(), true);
}

#[test]
fn gif_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/lena.gif").unwrap(), true);
}

#[test]
fn jpg_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/lena.jpg").unwrap(), true);
}

#[test]
fn tiff_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/palette-1c-8b.tiff").unwrap(), true);
}

#[test]
fn bmp_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/rgb-3c-8b.bmp").unwrap(), true);
}

#[test]
fn rgb_stream_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/pixelstream.rgb").unwrap(), true);
}

#[test]
fn gif2_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/null_file.gif").unwrap(), false);
}

#[test]
fn gif3_is_binary() {
    assert_eq!(binaryornot::is_binary("tests/isBinaryFile/trunks.gif").unwrap(), true);
}

#[test]
fn svg_is_not_binary() {
    assert_eq!(binaryornot::is_binary("tests/files/glyphiconshalflings-regular.svg").unwrap(), false);
}