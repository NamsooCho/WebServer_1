use std::fs::OpenOptions;
use std::{fs, io};
use std::io::{Read, Error};
use encoding_rs::{EUC_KR, CoderResult, UTF_8};

pub fn provide_text() -> Option<String> {
    let (encoding, _, _) = match detect_encoding_from("./resources/services/EOMI_euc_kr.txt") {
       Some(result) => { result }
        _ => { return None }
    };
    let mut decoder = match encoding.as_ref() {
        "EUC-KR" => EUC_KR.new_decoder(),
        _ => { UTF_8.new_decoder() }
    };

    let bytes = fs::read("./resources/services/EOMI_euc_kr.txt");
    let mut buffer_bytes = [0u8; 8];
    let buffer: &mut str = std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap();
    let mut bytes_in_buffer = 0usize;
    let mut output = String::new();
    let mut total_had_errors = false;

    let _ = bytes.map(|byte| {
        let mut total_read_from_current_input = 0usize;
        loop {
            let (result, read, written, had_errors) =
                decoder.decode_to_str(&byte[total_read_from_current_input..],
                                      &mut buffer[bytes_in_buffer..],
                                      false);
            total_read_from_current_input += read;
            bytes_in_buffer += written;
            total_had_errors |= had_errors;
            match result {
                CoderResult::InputEmpty => {
                    break;
                }
                CoderResult::OutputFull => {
                    output.push_str(&buffer[..bytes_in_buffer]);
                    bytes_in_buffer = 0usize;
                    continue;
                }
            }
        }
    });
    loop {
        let (result, _, written, had_errors) =
            decoder.decode_to_str(b"",
                                  &mut buffer[bytes_in_buffer..],
                                  true);
        bytes_in_buffer += written;
        total_had_errors |= had_errors;
        output.push_str(&buffer[..bytes_in_buffer]);
        bytes_in_buffer = 0usize;
        match result {
            CoderResult::InputEmpty => {
                break;
            }
            CoderResult::OutputFull => {
                continue;
            }
        }
    }
    return Some(output);
}

pub fn detect_encoding_from(filename: &str) -> Option<(String, f32, String)> {
    let mut file = match OpenOptions::new().read(true).open(filename).map(|x| x) {
        Ok(file)   => { file }, _ => { return None }
    };
    let mut reader: Vec<u8> = Vec::new();
    match file.read_to_end(&mut reader) {
        Ok(_) => { }, _ => return None
    };
    let result: (String, f32, String) = chardet::detect(&reader);
    return Some(result);
}

