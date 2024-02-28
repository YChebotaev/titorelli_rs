use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub enum ClassifiedType {
    Spam(f64),
    Ham(f64),
}

pub enum ExampleType {
    Spam,
    Ham,
}

#[derive(Default)]
struct Counter {
    ham: u64,
    spam: u64,
}

pub struct Model {
    stemmer: Stemmer,
    token_table: HashMap<String, Counter>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            stemmer: Stemmer::create(Algorithm::Russian),
            token_table: HashMap::new(),
        }
    }

    fn spam_total_count(&self) -> u64 {
        self.token_table.values().map(|x| x.spam).sum()
    }

    fn ham_total_count(&self) -> u64 {
        self.token_table.values().map(|x| x.ham).sum()
    }

    fn break_words(&self, example: &str) -> Vec<String> {
        example
            .unicode_words()
            .map(|word| word.to_lowercase())
            .map(|word| self.stemmer.stem(&word).into_owned())
            .collect()
    }

    fn rate_words(&self, example: &str) -> Vec<f64> {
        self.break_words(example)
            .into_iter()
            .map(|word| {
                if let Some(counter) = self.token_table.get(&word) {
                    if counter.spam > 0 && counter.ham == 0 {
                        return 0.99;
                    } else if counter.spam == 0 && counter.ham > 0 {
                        return 0.01;
                    } else if self.spam_total_count() > 0 && self.ham_total_count() > 0 {
                        let ham_prob = (counter.ham as f64) / (self.ham_total_count() as f64);
                        let spam_prob = (counter.spam as f64) / (self.spam_total_count() as f64);

                        return (spam_prob / (ham_prob + spam_prob)).max(0.01);
                    }
                }

                return 0.5 as f64;
            })
            .collect::<Vec<f64>>()
    }

    fn score_spam(&self, example: &str) -> f64 {
        let ratings = self.rate_words(example);

        let ratings = match ratings.len() {
            0 => return 0.0,
            x if x > 20 => {
                let length = ratings.len();
                let mut ratings = ratings;

                ratings.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

                [&ratings[..10], &ratings[length - 10..]].concat()
            }
            _ => ratings,
        };

        let product: f64 = ratings.iter().product();
        let alt_product: f64 = ratings.iter().map(|x| 1.0 - x).product();

        product / (product + alt_product)
    }

    pub fn classify(&self, example: &str) -> ClassifiedType {
        match self.score_spam(example) {
            s if s > 0.8 => ClassifiedType::Spam(s),
            s => ClassifiedType::Ham(s),
        }
    }

    pub fn train(&mut self, typ: ExampleType, example: &str) {
        let words = self.break_words(example);

        for word in words {
            let counter = self.token_table.entry(word).or_default();

            match typ {
                ExampleType::Ham => {
                    counter.ham += 1;
                }
                ExampleType::Spam => {
                    counter.spam += 1;
                }
            }
        }
    }
}
