use swappy::{find_anagrams, load_wordlist_from_file};
use rand::seq::SliceRandom;
use rand::thread_rng;

// use random_word::Lang;

fn main() {
    // let mut word_list: Vec<String> = random_word::all(Lang::En).iter()
    //     .map(|s| s.to_string())
    //     .collect();
    // word_list.retain(|word| word.len() >= 4);
    // let word = random_word::get(Lang::En);

    let mut word_list: Vec<String>  = load_wordlist_from_file("wordlist.txt").expect("Could not load wordlist");
    word_list.retain(|word| word.len() >= 4);

    let mut rng_word: rand::prelude::ThreadRng = thread_rng();
    let word: String = word_list
        .choose(&mut rng_word)
        .expect("Word list is empty")
        .to_string();

    let (_checked_nodes, anagrams   ) = 
        find_anagrams(&word, &word_list, 10, None, None).expect("Could not generate anagrams");

    let mut anagram_list: Vec<String> = anagrams.clone();
    anagram_list.retain(|x| x.trim() != &word);

    let mut rng_anagram: rand::prelude::ThreadRng = thread_rng();
    let anagram: String = anagram_list
        .choose(&mut rng_anagram)
        .expect("Word list is empty")
        .to_string();

    println!("Word to guess: {}", word);
    println!("Anagram found: {}", anagram);
    
}