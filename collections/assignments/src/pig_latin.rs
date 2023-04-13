pub fn print(text: &str) {
    let mut transformed_text = String::new();

    for word in text.split_whitespace() {
        if starts_with_vowel(word) {
            transformed_text.push_str(&(" ".to_owned() + word + &"-hay"))
        } else {
            transformed_text.push_str(&(" ".to_owned() + &transform_other(word)))
        }
    }

    let transformed_text = &transformed_text[1..];

    println!("===============================");
    println!("The original text:");
    println!("{text}");
    println!("The same text in pig latin:");
    println!("{transformed_text}");
}

fn starts_with_vowel(word: &str) -> bool {
    ['a', 'e', 'i', 'o', 'u', 'y']
        .iter()
        .any(|c| word.starts_with(*c))
}

fn transform_other(word: &str) -> String {
    let first_char = word.chars().nth(0).unwrap();
    let rest = &word[1..];

    format!("{rest}-{first_char}ay")
}
