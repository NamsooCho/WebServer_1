use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub fn process() {
    fn read_file(filename: &str) -> String {
        let mut contents = String::new();
        let mut file = File::open(filename).unwrap();
        let _ = file.read_to_string(&mut contents).unwrap();
        let length = (&contents).len();
        unsafe {
            if contents.starts_with("\u{feff}") {
                let contents = contents.get_unchecked_mut(3..length);
                // /*log/ */println!("{:?}", contents);
                return contents.to_string();
            }
        }
        return contents;
    }
    let contents = read_file("pride-and-prejudice.txt");
    // /*log/ */println!("{:?}", contents);

    fn filter_chars_and_normalize(contents: String) -> String {
        let s: String = contents.chars()
            .map(|x| match x.is_alphabetic() {
                false => ' ',
                _ => x.to_ascii_lowercase()
            }).collect();
        return s;
    }
    let str: String = filter_chars_and_normalize(contents);
    // /*log/ */println!("{}", str);

    fn remove_stop_words(str: String, filename: &str) -> Vec<String> {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents).unwrap();
        let stop_words: Vec<&str> = contents.split(",").collect::<Vec<&str>>();

        let words: Vec<&str> = str.split(" ").collect::<Vec<&str>>();

        let indexes: Vec<String> = words.clone().into_iter()
            .filter(|x| {
                !stop_words.contains(&x) && (x.to_string().len() != 0)
            })
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        return indexes;
    }
    let word_list = remove_stop_words(str, "stop_words.txt");

    fn frequencies(words: Vec<String>) -> HashMap<String, u16, RandomState> {
        let mut word_freq : HashMap<String, u16> = HashMap::new();
        for word in words {
            if word_freq.contains_key(&word) {
                *word_freq.get_mut(&word).unwrap() += 1;
            } else {
                word_freq.insert(word, 1);
            }
        }
        return word_freq;
    }
    let word_freq = frequencies(word_list);
    // /*log/ */println!("{:?}", word_freq);

    fn sort(word_freq: &HashMap<String, u16>) -> Vec<(&String, &u16)> {
        let pair: Vec<(&std::string::String, &u16)> = word_freq.iter().collect::<Vec<(&std::string::String, &u16)>>();
        let mut pair: Vec<(&std::string::String, &u16)> = pair.into_iter()
            .fold(Vec::new(), |mut acc: Vec<(&std::string::String, &u16)>, curr:(&std::string::String, &u16)| {
                let clon = curr.clone();
                acc.push(clon);
                return acc;
            });

        pair.sort_by(|a, b| { b.1.cmp(a.1) } );
        return pair;
    }
    let sorted_pair= sort(&word_freq);

    for p in sorted_pair.iter().enumerate() {
        // println!("{:?}", (*p).1);
        if p.0 <= 25 {
            println!("{:?} - {:?}", (*p.1).1, (*p.1).0);
        }
    }
}