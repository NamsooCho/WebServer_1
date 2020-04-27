use std::fs::File;
use std::io::Read;
use std::collections::{BTreeMap};
use std::iter::Iterator;

pub fn process_wordcount() {
    let contents = read_file("./resources/services/pride-and-prejudice.txt");
    debug!("{:?}", contents);

    let str: String = filter_chars_and_normalize(contents);
    debug!("{}", str);

    let word_list = remove_stop_words(str, "./resources/services/stop_words.txt");
    debug!("{:?}", word_list);

    let word_freq = frequencies(word_list);
    debug!("{:?}", word_freq);

    let sorted_pair = sort(&word_freq);
    debug!("{:?}", sorted_pair);

    for p in sorted_pair.iter().enumerate() {
        if p.0 <= 25 {
            info!("{:?} - {:?}", (*p.1).1, (*p.1).0);
        }
    }
}

fn read_file(filename: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(filename).expect("file does not exist");
    let _ = file.read_to_string(&mut contents).expect("file is not valid UTF-8");
    if contents.starts_with("\u{feff}") {
        let (_, right) = contents.split_at_mut(4);
        return right.to_string();
    }
    return contents;
}

fn filter_chars_and_normalize(contents: String) -> String {
    let s: String = contents.chars()
        .map(|x| match x.is_alphabetic() {
            false => ' ',
            _ => x.to_ascii_lowercase()
        }).collect();
    return s;
}

fn remove_stop_words(str: String, filename: &str) -> Vec<String> {
    let contents = read_file(filename);
    let stop_words: Vec<&str> = contents.split(",").collect::<Vec<&str>>();
    let alphabets: Vec<&str> = "abcdefghijklmnopqrstuvwxyz".split("").filter(|x| x.len() != 0 ).collect::<Vec<&str>>();
    let stop_words_with_alphabets = [&stop_words[..], &alphabets[..]].concat();

    let words: Vec<&str> = str.split(" ").collect::<Vec<&str>>();
    let indexes: Vec<String> = words.into_iter()
        .filter(|x| {
            !stop_words_with_alphabets.contains(x) && (x.to_string().len() != 0)
        })
        .map(|x| x.to_string())
        .collect();
    return indexes;
}

fn frequencies(words: Vec<String>) -> BTreeMap<String, u16> {
    let mut word_freq: BTreeMap<String, u16> = BTreeMap::new();
    for word in words {
        if word_freq.contains_key(&word) {
            *word_freq.get_mut(&word).unwrap() += 1;
        } else {
            word_freq.insert(word, 1);
        }
    }
    return word_freq;
}

fn sort(word_freq: &BTreeMap<String, u16>) -> Vec<(&String, &u16)> {
    let pair: Vec<(&std::string::String, &u16)> = word_freq.iter().collect::<Vec<(&std::string::String, &u16)>>();
    let mut pair: Vec<(&std::string::String, &u16)> = pair.into_iter()
        .fold(Vec::new(), |mut acc: Vec<(&std::string::String, &u16)>, curr: (&std::string::String, &u16)| {
            let clon = curr.clone();
            acc.push(clon);
            return acc;
        });

    pair.sort_by(|a, b| { b.1.cmp(a.1) });
    return pair;
}

