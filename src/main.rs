use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut doc1: String = String::new();
    let mut doc2: String = String::new();

    if args.len() < 3 {
        println!("enter file paths as argument");
        return;
    }

    read_lines(&mut doc1, args[1].as_str());
    read_lines(&mut doc2, args[2].as_str());

    if doc1.len() > 0 && doc2.len() > 0 {
        doc1 = doc1.replace("\n", " ");
        doc2 = doc2.replace("\n", " ");
        document_distance(&doc1, &doc2);
    }
}

fn read_lines(doc: &mut String, filename: &str) {
    let path: &Path = Path::new(filename);

    let mut file: File = match File::open(&path) {
        Err(why) => {
            println!("couldn't open {:?}: {}", path, why);
            return;
        }
        Ok(file) => file,
    };

    let mut s: String = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("couldn't read {:?}: {}", path, why);
            return;
        }
        Ok(_) => {}
    }

    *doc = s;
}

fn document_distance(doc1: &String, doc2: &String) {
    let mut word_count_1: HashMap<String, i32> = HashMap::new();
    let mut word_count_2: HashMap<String, i32> = HashMap::new();

    for word in doc1.split_whitespace() {
        let count: &mut i32 = word_count_1.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    for word in doc2.split_whitespace() {
        let count: &mut i32 = word_count_2.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    find_radian(&word_count_1, &word_count_2);
}

fn find_radian(vector_1: &HashMap<String, i32>, vector_2: &HashMap<String, i32>) {
    let len1: f64 = vector_length(vector_1);
    let len2: f64 = vector_length(vector_2);

    let mut dot_product: f64 = 0f64;

    for word in vector_1.keys() {
        let num1: f64 = match vector_1.get(word) {
            Some(count) => *count as f64,
            None => 0f64,
        };

        let num2: f64 = match vector_2.get(word) {
            Some(count) => *count as f64,
            None => 0f64,
        };

        dot_product += (num1 / len1) * (num2 / len2);
    }

    if dot_product > 1.0 {
        dot_product = 1.0;
    } else if dot_product < -1.0 {
        dot_product = -1.0;
    }

    println!("{} radians", dot_product.acos())
}

fn vector_length(vector: &HashMap<String, i32>) -> f64 {
    let mut length: f64 = 0.0;

    for word_count in vector.values() {
        length += (word_count * word_count) as f64
    }

    return length.sqrt();
}
