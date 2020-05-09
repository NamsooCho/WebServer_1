use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;

pub fn process_wordcount() {
    WordCounterController::new("./resources/services/pride-and-prejudice.txt").run();
}

#[derive(Debug)]
struct DataProvider {
    data: String
}

impl DataProvider {
    fn new(path_to_file: &str) -> DataProvider {
        let mut contents = String::new();
        let mut file = File::open(path_to_file).expect("file does not exist");
        let _ = file.read_to_string(&mut contents).expect("file is not valid UTF-8");

        let contents = match contents.starts_with("\u{feff}") {
            true => {
                let (_, right) = contents.split_at_mut(3);
                right.to_string()
            },
            false  => { contents }
        };

        let contents: String = contents.chars()
            .map(|x| match x.is_alphabetic() {
                false => ' ',
                _ => x.to_ascii_lowercase()
            }).filter(|x| {
            x.to_string().len() != 0
        }).collect();

        return DataProvider {
            data: contents
        }
    }
    fn words(&self) -> Vec<&str> {
        let res = self.data.split(" ").collect::<Vec<&str>>();
        return res;
    }
}


#[derive(Debug)]
struct StopWordProvider {
    stop_words: Vec<String>
}

impl StopWordProvider {
    fn new(path_to_file: &str) -> StopWordProvider {
        let mut contents = String::new();
        let mut file = File::open(path_to_file).expect("file does not exist");
        let _ = file.read_to_string(&mut contents).expect("file is not valid UTF-8");
        let contents = match contents.starts_with("\u{feff}") {
            true => {
                let (_, right) = contents.split_at_mut(3);
                right.to_string()
            },
            false  => { contents }
        };
        let stop_words: Vec<&str> = contents.split(",").collect::<Vec<&str>>();
        let alphabets: Vec<&str> = "abcdefghijklmnopqrstuvwxyz".split("").filter(|x| x.len() != 0 ).collect::<Vec<&str>>();
        let stop_words_with_alphabets = [&stop_words[..], &alphabets[..]].concat();

        return StopWordProvider {
            stop_words: stop_words_with_alphabets.iter().map(|x| { x.to_string() }).collect::<Vec<String>>()
        };
    }
    fn is_stop_word(&self, word: String) -> bool {
        return self.stop_words.contains(&word) || (word.len() == 0);
    }
}

#[derive(Debug)]
struct WordFrequencyCounter {
    word_freqs: BTreeMap<String, u16>
}

impl WordFrequencyCounter {
    fn new() -> WordFrequencyCounter {
        WordFrequencyCounter {
            word_freqs: Default::default()
        }
    }
    fn increment_count(&mut self, word: String) {
        if self.word_freqs.contains_key(&word) {
            *self.word_freqs.get_mut(&word).unwrap() += 1;
        } else {
            self.word_freqs.insert(word, 1);
        }
    }
    fn sorted(&self) -> Vec<(&String, &u16)> {
        let pair: Vec<(&std::string::String, &u16)> = self.word_freqs.iter().collect::<Vec<(&std::string::String, &u16)>>();
        let mut pair: Vec<(&std::string::String, &u16)> = pair.into_iter()
            .fold(Vec::new(), |mut acc: Vec<(&std::string::String, &u16)>, curr: (&std::string::String, &u16)| {
                let clon = curr.clone();
                acc.push(clon);
                return acc;
            });

        pair.sort_by(|a, b| { b.1.cmp(a.1) });
        return pair;
    }
}

#[derive(Debug)]
struct WordCounterController {
    data_provider: DataProvider,
    stop_word_provider: StopWordProvider,
    word_freq_counter: WordFrequencyCounter,
}

impl WordCounterController {
    fn new(path_to_file: &str) -> WordCounterController {
        WordCounterController {
            data_provider: DataProvider::new(path_to_file),
            stop_word_provider: StopWordProvider::new("./resources/services/stop_words.txt"),
            word_freq_counter: WordFrequencyCounter::new(),
        }
    }
    fn run(&mut self) {
        for word in self.data_provider.words() {
            if !self.stop_word_provider.is_stop_word(word.to_string()) {
                self.word_freq_counter.increment_count(word.to_string());
            }
        }
        let sorted_pair = self.word_freq_counter.sorted();

        for p in sorted_pair.iter().enumerate() {
            if p.0 <= 25 {
                println!("{:?} - {:?}", (*p.1).1, (*p.1).0);
            }
        }
    }
}

