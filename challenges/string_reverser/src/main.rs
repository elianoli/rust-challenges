fn main() {
    println!("{}", string_reverser("Hello World!"));
}

fn string_reverser(word: &str) -> String {
    let mut word_reversed = String::from("");
    for char in word.chars() {
        word_reversed.insert_str(0, &format!("{char}").to_string());
    }
    word_reversed
}
