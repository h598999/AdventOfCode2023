use std::{fs::File, io::{BufRead, BufReader}, fmt::format};

fn get_string_value(line: &str) -> i32 {
    let value:i32 = 0;
    let mut first: char = 'e';
    let mut last: char = 'e';

    for (index, c) in line.chars().enumerate(){
        if c.is_numeric(){
            first = c;
            break;
        } else {
            first ='e';
        }
    }

    for (index, c) in line.char_indices().rev(){
        if c.is_numeric(){
            last = c;
            break;
        } else {
            last = 'c';
        }
    }
    
    let end_res: String = format!("{}{}", first, last);
    match end_res.parse::<i32>(){
        Ok(value) => {
            return value;
        }
        Err(e) => {
            println!("Error: {}",e);
            return 0;
        }
    }

}

fn main() {
    let file = File::open("./src/input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut result: i32 = 0;

    for line in reader.lines(){
        match line{
            Ok(line) => {
                let temp: i32 = get_string_value(&line);
                result += temp;
            }
            Err(err) => {
                eprintln!("Error reading lines: {}",err);
            }
        }
    }
    println!("Parsed Number: {}", result);
}

