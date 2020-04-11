use std::io;
fn main() {
    let mut cipher_text = String::new();
    const UPPERCASE_BEG: u32 = 65;
    const LOWERCASE_BEG: u32 = 97;
    println!("Enter Cipher Text ");
    io::stdin().read_line(&mut cipher_text)
        .expect("Failed to Read line");
    let cipher_text = cipher_text.trim();
    println!("Possible Plain Texts");
    for shift in 1..27 {
        let mut plain_text = String::new();
        for c in cipher_text.chars() {
            if !c.is_alphabetic() {
                plain_text.push(c);
            } else if c.is_ascii_uppercase() {
                let shifted_c = shift_char(c, UPPERCASE_BEG, 26 - shift);
                plain_text.push(shifted_c);
            } else if c.is_ascii_lowercase() {
                let shifted_c = shift_char(c, LOWERCASE_BEG, 26 - shift); plain_text.push(shifted_c);
            }
        }
        println!("Shift: {} ==> {}", shift, plain_text);
    }
}

fn shift_char(c: char, start: u32, shift: u32) -> char {
    std::char::from_u32((((c as u32 - start) + shift) % 26) + start).unwrap()
}
