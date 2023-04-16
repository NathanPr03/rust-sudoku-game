use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use rand::Rng;

pub struct Trivia {
    trivia_questions: HashMap<String, String>,
}

impl Trivia {
    pub fn new() -> Trivia
    {
        let trivia_questions = read_from_text_file();
        return Trivia
        {
            trivia_questions,
        };
    }

    pub fn get_trivia_question_and_answer(&self) -> (String, String)
    {
        let mut random_num_generator = rand::thread_rng();

        let random_number: usize = random_num_generator.gen_range(0..95) as usize;

        let keys: Keys<String, String> = self.trivia_questions.keys();

        let mut key_string: String = "Question".to_string();
        let mut counter = 0;

        for key in keys {
            if random_number == counter {
                key_string = key.to_string();
            }
            counter = counter + 1;
        }

        return (key_string.clone(), self.trivia_questions[&key_string].clone());
    }
}

fn read_from_text_file() -> HashMap<String, String>
{
    let mut trivia_questions = HashMap::with_capacity(100);
    let lines = read_lines("./src/trivia.txt".to_string());
    for line in lines {
        let unwrapped_line = line.unwrap();
        let split: Vec<&str> = unwrapped_line.split(".").collect();
        trivia_questions.insert(split[0].to_string(), split[1].trim().to_string());
    }

    return trivia_questions;
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return BufReader::new(file).lines();
}