use clap::{Args, Subcommand};

use std::cmp;
use std::collections::HashMap;

use std::fs::{read_to_string, File};
use std::io::{prelude::*, BufReader};

#[derive(Debug, Args)]
pub struct Chapter1Args {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    PatternCount(PatternCountArgs),
    BetterFrequentWords(BetterFrequentWordsArgs),
    ReverseCompliment(ReverseComplimentArgs),
    PatternMatchProblem(PatternMatchProblemArgs),
}

#[derive(Debug, Args, Clone)]
struct PatternCountArgs {
    // The file contain the text to search
    path: String,
}

#[derive(Debug, Args, Clone)]
struct BetterFrequentWordsArgs {
    // The file contain the text to search
    path: String,
}

#[derive(Debug, Args, Clone)]
struct ReverseComplimentArgs {
    // The file contain the text to search
    path: String,
}

#[derive(Debug, Args, Clone)]
struct PatternMatchProblemArgs {
    // The file contain the text to search
    path: String,
}

fn pattern_count(text: &String, pattern: &String) -> i32 {
    let mut val = 0;
    let pattern_len = pattern.len();
    for i in 0..(text.len() - pattern_len + 1) {
        if pattern.eq(&text[i..i + pattern_len]) {
            val += 1;
        }
    }
    return val;
}

fn frequency_table(text: &String, k: i32) -> HashMap<String, i32> {
    let mut frequency_map: HashMap<String, i32> = HashMap::new();
    // Pregenerate all permutation of the k-mer
    let alphabet = vec!["A", "T", "C", "G"];
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
    for i in 0..(text.len() - k_usize) {
        let kmer = text[i..i + k_usize].to_owned();
        //println!("| {} {} |", kmer, kmer.len());
        if kmer.len() != k_usize {
            continue;
        }
        let val = frequency_map.get(&kmer).unwrap();
        frequency_map.insert(kmer, val + 1);
    }
    return frequency_map;
}

fn max_map(maps: &HashMap<String, i32>) -> i32 {
    let mut current_min = i32::MIN;
    for (_key, value) in maps {
        current_min = cmp::max(current_min, *value);
    }
    return current_min;
}

fn better_frequent_words(text: &String, k: i32) -> Vec<String> {
    let frequency_map = frequency_table(text, k);
    let max = max_map(&frequency_map);
    let mut frequent_patterns = Vec::new();
    for (key, value) in frequency_map {
        if max == value {
            frequent_patterns.push(key)
        }
    }
    return frequent_patterns;
}

fn compliment_dna(text: &String) -> String {
    return text.chars().map(|letter| match letter {
        'A' => 'T',
        'C' => 'G',
        'T' => 'A',
        'G' => 'C',
        _ => 'X'
    }).collect();
}

fn pattern_match_indexes(text: &String, pattern: &String) -> Vec<usize> {
    let mut index = Vec::new();
    let pattern_len = pattern.len();
    for i in 0..(text.len() - pattern_len + 1) {
        if pattern.eq(&text[i..i + pattern_len]) {
            index.push(i)
        }
    }
    return index;
}


pub fn chapter1_command_runner(stuff: Chapter1Args) {
    match stuff.commands {
        Commands::PatternCount(args) => {
            let file = File::open(args.path).unwrap();
            let mut reader = BufReader::new(file);
            let mut text = String::new();
            reader.read_line(&mut text).unwrap();
            text.pop();
            text.pop();
            let mut pattern = String::new();
            reader.read_line(&mut pattern).unwrap();
            pattern.pop();
            pattern.pop();
            println!("{}", pattern_count(&text, &pattern));
        }
        Commands::BetterFrequentWords(args) => {
            let file = File::open(args.path).unwrap();
            let mut reader = BufReader::new(file);
            let mut text = String::new();
            reader.read_line(&mut text).unwrap();
            text.pop();
            text.pop();
            let mut k_len_thing = String::new();
            reader.read_line(&mut k_len_thing).unwrap();
            for thing in better_frequent_words(&text, k_len_thing.parse().unwrap()) {
                print!(" {}", thing);
            }
        }
        Commands::ReverseCompliment(args) => {
            let mut line = read_to_string(args.path).unwrap();
            line.pop();
            line.pop();
            println!("{}", compliment_dna(&line).chars().rev().collect::<String>());
        },
        Commands::PatternMatchProblem(args) => {
            let file = File::open(args.path).unwrap();
            let mut reader = BufReader::new(file);
            let mut pattern = String::new();
            reader.read_line(&mut pattern).unwrap();
            pattern.pop();
            pattern.pop();
            let mut sequence = String::new();
            reader.read_line(&mut sequence).unwrap();
            sequence.pop();
            sequence.pop();
            for thing in pattern_match_indexes( &sequence, &pattern) {
                print!(" {}", thing);
            }
        }
    }
}
