
pub fn encode(clear_text: &str, key: char) -> String {
    let mut new_text = String::new();

    let k = key as u8 - 97;
    for current in clear_text.chars() {
        let x = current as u8 - 97;
         new_text.push(((k + x) % 26 + 97) as char);
    }

    return new_text;
}
