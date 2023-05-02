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

fn main() {
    //creating the Vector, where we store out lines into
    let mut line_text_vector: Vec<String> = Vec::new();

    //reading the file and storing each line as a vector and
    if let Ok(lines) = read_lines("games.txt") {
        for line in lines {
            if let Ok(line_text) = line {
                line_text_vector.push(line_text);
            }
        }
    }

    //generating a random number between Zero and the Vector Length
    let mut rng = thread_rng();
    let random_number: usize = rng.gen_range(0..line_text_vector.len());

    //selecting the Vector Value with the generated random number
    let vector_to_trim = line_text_vector.get(random_number);

    //trimming the vector and printing it
    println!("{}", trimable_vector(vector_to_trim));
}