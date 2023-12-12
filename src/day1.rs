use std::{fs::File, io::{BufRead, BufReader}, char};

fn get_string_value(line: &str) -> i32 {
    let _value:i32 = 0;
    let mut first: char = 't';
    let mut last: char = 't';
    let mut first_str: String = String::new();
    let mut last_str: String = String::new();
     let spelled_out_mapping: Vec<(&str, char)> = vec![
        ("one", '1'),
        ("two", '2'),
        ("three",'3'),
        ("four",'4'),
        ("five",'5'),
        ("six",'6'),
        ("seven",'7'),
        ("eight",'8'),
        ("nine",'9'),
        ("fourteen",'4'),
        ("sixteen",'6'),
        ("seventeen",'7'),
        ("eighteen",'8'),
        ("nineteen",'9'),
    ];

    for (_index, c) in line.chars().enumerate(){
        if c.is_numeric(){
            first = c;
            break;
        } else {
            first_str.push(c);
            // Check if any substring of first_str is a key in the vector
            if let Some(&(_, value)) = spelled_out_mapping.iter().find(|(s, _)| first_str.contains(s)) {
                first = value;
                break;
            }
        }
    }

    for (_index, c) in line.char_indices().rev(){
        if c.is_numeric(){
            last = c;
            break;
        } else {
            last_str.insert(0, c);
            if let Some(&(_, value)) = spelled_out_mapping.iter().find(|(s, _)| last_str.contains(s)) {
                last = value;
                break;
            }
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
    let file = File::open("./src/inputs/input_day1.txt").expect("File not found");
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
