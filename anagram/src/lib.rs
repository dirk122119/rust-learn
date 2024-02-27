use std::collections::HashSet;
use std::collections::HashMap;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut finds: HashSet<&str> = HashSet::new();
    for anagram in possible_anagrams {
        let lowercase_word = word.to_lowercase();
        let  lowercase_anagram = anagram.to_lowercase();

        if anagram.len() == word.len() && lowercase_word != lowercase_anagram {
            let mut hashtable_word = HashMap::new();
            let mut hashtable_anagram = HashMap::new();
            for c in lowercase_word.chars() {
                let count = hashtable_word.entry(c).or_insert(0);
                *count += 1;
            }
            for c in lowercase_anagram.chars() {
                let count = hashtable_anagram.entry(c).or_insert(0);
                *count += 1;
            }
            if hashtable_word == hashtable_anagram {
                finds.insert(anagram);
            }
        }
            
    }
    finds
}
