use std::fs::{File, read_dir};
use std::io::Read;

use crate::{decode_utf8, encode_utf8};

lazy_static! {
    static ref TEST_TEXTS: Vec<(String, Vec<char>)> = collect_test_texts();
}

fn collect_test_texts() -> Vec<(String, Vec<char>)> {
    let test_text_dir = "./test_text";

    let mut result = Vec::new();
    let dir = read_dir(test_text_dir).expect("read test text directory");
    for file in dir {
        let file = file.expect("walk in test text directory");
        if file.file_type().expect("get file type").is_file() {
            let mut file = File::open(file.path()).expect("open test file");
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).expect("read test file");

            let chars: Vec<char> = buffer.chars().collect();
            result.push((buffer, chars));
        }
    }
    result
}

#[test]
fn test_encode() {
    for (text, chars) in TEST_TEXTS.iter() {
        assert_eq!(encode_utf8(&chars).unwrap(), text.as_bytes());
    }
}

#[test]
fn test_encode_error() {
    let mut src = vec!['a', 'b', 'c'];
    let invalid_char = unsafe { std::mem::transmute::<u32, char>(1 << 21) };
    src.push(invalid_char);

    let output = encode_utf8(&src);
    assert!(output.is_err());
    assert_eq!(output.unwrap_err().index, 3);
}

#[test]
fn test_decode() {
    for (text, chars) in TEST_TEXTS.iter() {
        assert_eq!(&decode_utf8(text.as_bytes()).unwrap(), chars);
    }
}

#[test]
fn test_decode_error() {
    let mut src = "学习".to_string().into_bytes();
    src.remove(0);

    let output = decode_utf8(&src);
    assert!(output.is_err());
    let error = output.unwrap_err();
    assert_eq!(error.index, 0);
}
