use crate::encode::encode;

pub fn decode(ciphered_text: &String, key: &char) -> String {
    let reversed_key = (26 - (*key as u8 - 97) + 97) as char;
    return encode(&ciphered_text, &reversed_key);
}
