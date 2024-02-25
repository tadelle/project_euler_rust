//! XOR Decryption
//! 
//! Each character on a computer is assigned a
//! unique code and the preferred standard is 
//! ASCII (American Standard Code for Information 
//! Interchange). For example, uppercase A = 65,
//! asterisk (*) = 42, and lowercase k = 107.
//! 
//! A modern encryption method is to take a text file,
//! convert the bytes to ASCII, then XOR each byte
//! with a given value, taken from a secret key.
//! The advantage with the XOR function is that using
//! the same encryption key on the cipher text,
//! restores the plain text; for example,
//! 65 XOR 42 = 107, then 107 XOR 42 = 65.
//! 
//! For unbreakable encryption, the key is the same
//! length as the plain text message, and the key is
//! made up of random bytes. The user would keep the
//! encrypted message and the encryption key in different
//! locations, and without both "halves", it is
//! impossible to decrypt the message.
//! 
//! Unfortunately, this method is impractical for most
//! users, so the modified method is to use a password
//! as a key. If the password is shorter than the message,
//! which is likely, the key is repeated cyclically
//! throughout the message. The balance for this method
//! is using a sufficiently long password key for security,
//! but short enough to be memorable.
//! 
//! Your task has been made easy, as the encryption key
//! consists of three lower case characters. Using 0059_cipher.txt
//! (right click and 'Save Link/Target As...'), a file containing
//! the encrypted ASCII codes, and the knowledge that the plain
//! text must contain common English words, decrypt the message
//! and find the sum of the ASCII values in the original text.
use std::fs;

use super::Problem;

crate::base_problem!(129448, "XOR Decryption");

fn get_result_problem() -> i64 {

    let vec = get_data();
    let keys = get_keys();

    let mut sum: i64 = 0;
    for key in keys.into_iter() {
        
        let mut index = 0;
        
        for n in vec.iter() {
            index += 1;
            let nxor = 
                match index % 3 {
                    1 => key.0 as u8 ^ n,
                    2 => key.1 as u8 ^ n,
                    0 => key.2 as u8 ^ n,
                    _ => 0
            };

            if is_english_text(nxor) {
                sum += nxor as i64;
            } else {
                sum = 0;
                break;
            }
        }
        if sum > 0 {
            break;
        }
    }

    sum
}

fn get_data() -> Vec<u8>{
    fs::read_to_string("src/data/0059_cipher.txt")
    .unwrap_or("".to_string())
    .split(',')
    .map(|n| n.parse::<u8>().unwrap_or(0))
    .collect::<Vec<u8>>()
}

fn get_keys() -> Vec<(char, char, char)> {
    let mut keys: Vec<(char, char, char)> = Vec::new();

    for c1 in 'a'..='z' {
        for c2 in 'a'..='z' {
            for c3 in 'a'..='z' {
                keys.push((c1, c2, c3));
            }
        }
    }
    keys
}

fn is_english_text(character: u8) -> bool {

    match character {
        c if c.is_ascii_alphanumeric() => true,
        c if c.is_ascii_whitespace() => true,
        c if c.is_ascii_punctuation() => {
            !matches!(c, b'\xB4' | b'`' | b'|')
        },
        _ => false
    }
}
