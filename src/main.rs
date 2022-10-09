use clap::Parser;
use clap::{Args, Subcommand};

use std::collections::HashMap;
use std::cmp;
use std::fs;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[clap(
    name = "Bioinformatics Algorithms: An active learning guide",
    version = "1.0.0",
    author = "Inura De Zoysa, github: inuradz",
    about = "Stores all the examples in an easy to access way"
)]
struct Cli {
    /// The pattern to look for
    #[clap(subcommand)]
    chapter: Chapters,
}

#[derive(Debug, Subcommand)]
enum Chapters {
    Chapter1(Chapter1Args),
}

#[derive(Debug, Args)]
struct Chapter1Args {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    PatternCount(PatternCountArgs),
    BetterFrequentWords(BetterFrequentWordsArgs)
}

#[derive(Debug, Args, Clone)]
struct PatternCountArgs {
    // The file contain the text to search
    path: String,
    // The pattern to search in the file
    pattern: String
}

#[derive(Debug, Args, Clone)]
struct BetterFrequentWordsArgs {
    // The file contain the text to search
    path: String
}

fn pattern_count(text : &String, pattern: &String) -> i32 {
    let mut val = 0;
    let pattern_len = pattern.len();
    for i in 0..(text.len()-pattern_len+1) {
        if pattern.eq(&text[i..i+pattern_len]) {
            val += 1;
        }
    }
    return val
}

fn frequency_table(text: &String, k: i32) -> HashMap<String,i32> {
    let mut frequency_map : HashMap<String, i32> = HashMap::new();
    // Pregenerate all permutation of the k-mer
    let alphabet = vec!["A","T","C","G"];
    let mut all_k_mers: Vec<String> = Vec::new();
    // TODO: Do not pregenerate all combinations in advance
    for letter in alphabet.clone() {
        all_k_mers.push(letter.to_string());
    }
    for _ in 1..k {
        let mut temp = Vec::new();
        for kmer in all_k_mers {
            for letter in alphabet.clone() {
                temp.push(kmer.to_owned() + letter)
            }
        }
        all_k_mers = temp;
    }

    for kmer in all_k_mers {
        //println!("| {} {} |", &kmer, kmer.len());
        frequency_map.insert(kmer, 0);
    }
    let k_usize = k as usize;
    for i in 0..(text.len()-k_usize) {
        let kmer =  text[i..i+k_usize].to_owned();
        //println!("| {} {} |", kmer, kmer.len());
        if kmer.len() != k_usize {
            continue;
        }
        let val = frequency_map.get(&kmer).unwrap();
        frequency_map.insert(kmer, val + 1);
    }
    return frequency_map;
}

fn max_map(maps: &HashMap<String,i32>) -> i32 {
    let mut current_min = i32::MIN;
    for (_key,value) in maps {
        current_min = cmp::max(current_min,*value);
    }
    return current_min;
}

fn better_frequent_words(text: &String, k:i32) -> Vec<String> {
    let frequency_map = frequency_table(text,k);
    let max = max_map(&frequency_map);
    let mut frequent_patterns = Vec::new();
    for (key,value) in frequency_map {
        if max == value {
            frequent_patterns.push(key)
        }
    } 
    return frequent_patterns;
}

fn main() {
    let args = Cli::parse();
    match args.chapter {
        Chapters::Chapter1(chapter1)  => 
        match chapter1.commands {
            Commands::PatternCount(args) => {
                println!("{}",pattern_count(&fs::read_to_string(args.path).unwrap(), &args.pattern));
            },
            Commands::BetterFrequentWords(args) => {
                let file = File::open(args.path).unwrap();
                let mut reader = BufReader::new(file);
                let mut text = String::new();
                reader.read_line(&mut text);
                text.pop();
                let mut k_len_thing = String::new();
                reader.read_line(&mut k_len_thing);
                for thing in better_frequent_words(&text, k_len_thing.parse().unwrap()) {
                    print!(" {}", thing);
                }
            }
        }   
    }
}
