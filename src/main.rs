use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::{thread_rng, Rng};

use std::fmt;

//function to trim our vector for print output
pub fn trimable_vector<I, D>(trimable: I) -> String
where
    I: IntoIterator<Item =D>,
    D: fmt::Display,
    {
        let mut trimable = trimable.into_iter();

        let head = match trimable.next() {
            None => return String::from("[]"),
            Some(x) => format!("{}", x),
        };
        let body = trimable.fold(head, |a, v| format!("{}, {}", a, v));
        format!("{}", body)
    }

//function to read the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

fn create_random_number(lines: usize) -> usize {
    //generating a random number between Zero and the Vector Length
    let mut rng = thread_rng();
    let random_number: usize = rng.gen_range(0..lines);
    random_number
    }

fn store_lines_into_vector() -> Vec<String>{
    let mut line_text_vector: Vec<String> = Vec::new();
        //reading the file and storing each line into the vector
        if let Ok(lines) = read_lines("games.txt") {
            for line in lines {
                if let Ok(line_text) = line {
                    line_text_vector.push(line_text);
                }
            }
        }
    return line_text_vector
}

fn main() {
    println!("{}", trimable_vector(store_lines_into_vector().get(create_random_number(store_lines_into_vector().len()))));
}