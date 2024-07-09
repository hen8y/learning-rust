use std::io;


fn permutate(words: &str) -> Vec<String> {
    let word_chars: Vec<char> = words.chars().collect();
    let mut result: Vec<String> = Vec::new();
    gen_permutation(&word_chars, 0, &mut result);
    return result;
}

fn gen_permutation(word_chars: &Vec<char>, index: usize, result: &mut Vec<String>) {
    if word_chars.len() == index  {
        result.push(word_chars.iter().collect());
    } else {
        let mut chars_clone = word_chars.clone();
        for i in index..chars_clone.len() {
            chars_clone.swap(index, i);
            gen_permutation(&chars_clone, index + 1, result);
            chars_clone.swap(index, i);
        }
    }
}



pub fn unscrabble() {
    println!("Enter a 4 letter word");
    let mut word_input = String::new();

    io::stdin()
        .read_line(&mut word_input)
        .expect("Failed to read input");
    let words = word_input.trim();
    let results =  permutate(words);
    println!("\ngenerated words: ");
    for new_word in results{
        println!("{new_word}")
    }
}



