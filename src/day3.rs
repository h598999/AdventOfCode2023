use std::io::{ BufReader, BufRead};
use std::fs::File;
pub mod day3{
    use super::*;

    fn sum_of_a_line(line: &str) -> i32{
        let mut temp: String = String::new();
        let mut table: Vec<i32> = Vec::new();
        for (_index, ch) in line.chars().enumerate(){
            if ch.is_numeric(){
                temp.push(ch);
            } else {
                if !temp.is_empty(){
                    table.push(temp.parse().unwrap_or(0));
                    temp = String::new();
                }
            }
        }
        return table.iter().sum();
    }

    fn should_sum(current_line: &str) -> bool{

        for (_index, ch) in current_line.chars().enumerate(){
            if ch.is_ascii_punctuation(){
                if !ch.eq(&'.'){
                    return true;
                }
            }
        }
        return false;
    }
    pub fn solution(){
        let file = File::open("./src/inputs/inputTest_day3.txt").expect("File not found");
        let reader = BufReader::new(file);
        let mut result:i32 = 0;
        let mut temp_val: i32 = 0;
        let mut line_counter: i32 = 1;
        for line in reader.lines(){
            match line{
                Ok(line) => {
                    if should_sum(&line){
                        println!("Should sum line: {}",line_counter-1);
                        println!("Adding {} from line{}", temp_val,line_counter-1);
                        result+=temp_val;
                    } 
                    temp_val = sum_of_a_line(&line);
                    println!("Value of line{}: {}",line_counter, temp_val);
                }
                Err(err) => {
                    eprintln!("Error reading lines: {}",err);
                }
            }
           line_counter = line_counter+1; 
        }
        println!("Sum: {}", result);
    }
}
