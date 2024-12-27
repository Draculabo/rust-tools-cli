use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};

use crate::cli::Base64Format;
fn get_file_content(input: String) -> anyhow::Result<String> {
    let mut reader: Box<dyn std::io::Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let buffer = buffer.trim();
    Ok(buffer.to_string())
}
pub fn process_base64_encode(input: String, format: Base64Format) -> anyhow::Result<String> {
    let buffer = get_file_content(input)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
    };
    println!("{}", encoded);
    Ok(encoded)
}

pub fn process_base64_decode(input: String, format: Base64Format) -> anyhow::Result<String> {
    let buffer = get_file_content(input)?;
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
    };
    let decoded_str = String::from_utf8(decoded)?;
    println!("{}", decoded_str);
    Ok(decoded_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_file_content() {
        let input = "fixtures/base64_result_standard.txt".to_string();
        let content = get_file_content(input).unwrap();
        println!("{}", content);
    }
    #[test]
    fn test_process_base64_encode_standard() {
        let input = "fixtures/base64_origin_standard.txt".to_string();
        let encoded = process_base64_encode(input, Base64Format::Standard).unwrap();
        assert_eq!(encoded, "aGVsbG8=");
        println!("{}", encoded);
    }
    #[test]
    fn test_process_base64_encode_url_safe() {
        let input = "fixtures/base64_origin_urlsafe.txt".to_string();
        let encoded = process_base64_encode(input, Base64Format::UrlSafe).unwrap();
        assert_eq!(encoded, "aGVsbG8");
        println!("{}", encoded);
    }
    #[test]
    fn test_process_base64_decode_standard() {
        let input = "fixtures/base64_result_standard.txt".to_string();
        let decoded = process_base64_decode(input, Base64Format::Standard).unwrap();
        assert_eq!(decoded, "hello");
        println!("{}", decoded);
    }
    #[test]
    fn test_process_base64_decode_url_safe() {
        let input = "fixtures/base64_result_urlsafe.txt".to_string();
        let decoded = process_base64_decode(input, Base64Format::UrlSafe).unwrap();
        assert_eq!(decoded, "hello");
        println!("{}", decoded);
    }
}
