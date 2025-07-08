pub struct RepeatingCharacter {}

impl RepeatingCharacter {
    pub fn repeating_character(s: String) -> char {
        let mut list: Vec<char>;
        list = s.chars().collect();
        for (i, ch) in s.chars().enumerate() {
            let element = list[i];
            list.remove(i);
            if list.contains(&element) {
                return element;
            } else {
                list.push(element);
            }
        }
        '$'
    }
}