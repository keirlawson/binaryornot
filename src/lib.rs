use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use chardet;
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;

const STARTING_CHUNK_SIZE: u64 = 1024;

/// Test whether or not a file at a given path is a binary.
/// 
/// Example:
/// ```no_run
/// use binaryornot;
///
/// if binaryornot::is_binary("/path/to/some/file").expect("unable to read file") {
///    println!("a binary!");
/// }
/// ```
pub fn is_binary<P: AsRef<Path>>(path: P) -> Result<bool> {
    let chunk = get_starting_chunk(path, STARTING_CHUNK_SIZE)?;
    Ok(is_binary_string(chunk))
}

fn get_starting_chunk<P: AsRef<Path>>(path: P, limit: u64) -> Result<Vec<u8>> {
    let f = File::open(path)?;
    let mut f = f.take(limit);
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn is_printable_high_ascii(value: &u8) -> bool {
    match value {
        127 ..= 255 => true,
        _ => false
    }
}

fn is_printable_ascii(value: &u8) -> bool {
    match value {
        32 ..= 126 => true,
        b'\n' | b'\r' | b'\t' | b'\x0C' | b'\x08' => true,
        _ => false
    }
}

fn decoding_possible(bytes_to_check: &Vec<u8>, detected_encoding: String) -> bool {
    let coder = encoding_from_whatwg_label(chardet::charset2encoding(&detected_encoding));
    if let Some(coder) = coder {
        coder.decode(&bytes_to_check, DecoderTrap::Strict).is_ok()
    } else {
        false
    }
}

fn contains_null_bytes(bytes: &Vec<u8>) -> bool {
    bytes.contains(&b'\x00') || bytes.contains(&b'\xff')
}

fn is_binary_string(bytes_to_check: Vec<u8>) -> bool {

    // Empty files are considered text files
    if bytes_to_check.is_empty() {
        return false;
    }

    // Now check for a high percentage of ASCII control characters
    // Binary if control chars are > 30% of the string
    let non_low_char_count = bytes_to_check.iter().filter(|x| !is_printable_ascii(x)).count();
    let nontext_ratio1 = (non_low_char_count as f64) / (bytes_to_check.len() as f64);

    // and check for a low percentage of high ASCII characters:
    // Binary if high ASCII chars are < 5% of the string
    // From: https://en.wikipedia.org/wiki/UTF-8
    // If the bytes are random, the chances of a byte with the high bit set
    // starting a valid UTF-8 character is only 6.64%. The chances of finding 7
    // of these without finding an invalid sequence is actually lower than the
    // chance of the first three bytes randomly being the UTF-8 BOM.
    let non_high_char_count = bytes_to_check.iter().filter(|x| !is_printable_high_ascii(x)).count();
    let nontext_ratio2 = (non_high_char_count as f64) / (bytes_to_check.len() as f64);

    let is_likely_binary = 
        (nontext_ratio1 > 0.3 && nontext_ratio2 < 0.05) ||
        (nontext_ratio1 > 0.8 && nontext_ratio2 > 0.8)
    ;

    // then check for binary for possible encoding detection with chardet
    let (detected_encoding, detected_encoding_confidence, ..) = chardet::detect(&bytes_to_check);

    //finally use all the check to decide binary or text
    let decodable_as_unicode = if detected_encoding_confidence > 0.9 && detected_encoding != "ascii" {
        decoding_possible(&bytes_to_check, detected_encoding)
    } else {
        false
    };

    if is_likely_binary {
        !decodable_as_unicode
    } else {
        if decodable_as_unicode {
            false
        } else {
            contains_null_bytes(&bytes_to_check)
        }
    }
}