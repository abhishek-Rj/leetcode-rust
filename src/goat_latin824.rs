pub struct Solution {} 

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowel : Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut words: Vec<String> = sentence.split(' ').map(|s| s.to_string()).collect();
        for i in 0..words.len() {
            match words[i].chars().nth(0) {
                Some(tt) => {
                    if !vowel.contains(&tt) {
                        words[i].remove(0);
                        words[i].push(tt);                        
                    }
                    words[i].push('m');
                    words[i].push('a');
                    let mut iter = 1;
                    while iter <= i + 1 {
                        words[i].push('a');
                        iter = iter + 1;
                    }
                },
                None => panic!("Hell no")
            }
        }
        let mut new_sentence = String::from("");
        for i in 0..words.len() {
            new_sentence.push_str((words[i]).as_str());
            if !(i == words.len() - 1) {
                new_sentence.push(' ');
            }
        }  
        new_sentence
    }
}