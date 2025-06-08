fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    //creating a map for storing counts of vowels
    let mut vowel_counts = std::collections::HashMap::new();

    // iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => *vowel_counts.entry(c).or_insert(0) += 1,
            _ => continue,
        }
    }

    for (key, value) in &vowel_counts {
        println!("{}: {}", key, value);
    }

    println!(
        "Longest word in sentence: {}\nIs: {}",
        sentence,
        longest_word_in_sentence(&sentence)
    );

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}

fn longest_word_in_sentence(sentence: &str) -> &str {
    //longest word using iterator
    sentence
        .split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap()
}
