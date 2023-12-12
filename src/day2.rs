use std::io::{BufRead,BufReader};
use std::fs::File;
pub mod day2 {
    use std::{collections::HashMap};

    use super::*;

    fn str_to_i32(str: &str) -> i32{
        str.parse().unwrap_or(0)
    }

    fn find_substring(line: &str, start_char: char, end_char: char) -> &str{ 
        match (line.find(start_char), line.rfind(end_char)) {
        (Some(start), Some(end)) if start < end => {
            let start_index = start + 1;
            let end_index = end;
            let substring: &str = &line[start_index..end_index];
            substring
        }
        _ => "",
    }
    }

    fn find_suffix(line: &str, start_char: char) -> &str{
        match (line.find(start_char), line.len()) {
            (Some(start), end) if start < end => {
                let substring: &str = &line[start+1..end];
                substring
            }
            _ => "",
        }
    }

    fn find_suffix_from_prefix<'a>(input: &'a str, prefix: &'a str) -> &'a str {
    if let Some(index) = input.find(prefix) {
        &input[index + prefix.len()..]
    } else {
        input
    }
    }

    fn possible(line: &str) -> i32{
        let game_id:i32 = str_to_i32(find_substring(line, ' ', ':'));
        let game: &str = find_suffix(line, ':');
        let handfuls: Vec<&str> = game.split(";").collect();
        let mut i = 1;
        for hf in handfuls {
            i = i+1;
            // println!("hf: {}",hf);
            let sets: Vec<&str> = hf.split(",").collect();
            for set in sets {
                // println!("set: {}", set);
                let value_str: &str = find_substring(set,' ',' ').trim();
                let color:&str = find_suffix_from_prefix(set, value_str).trim();
                let value = str_to_i32(value_str);
                // print!("Value:{}", value);
                // println!(" Color:{}", color);
                //
                if value > 12 && color.eq("red"){
                    return 0;
                }

                if value > 13 && color.eq("green"){
                    return 0
                }
                if value > 14 && color.eq("blue"){
                    return 0
                }
            }
        }
        return game_id;
    }

    fn cube_power(line: &str) -> i32{
        let game: &str = find_suffix(line, ':');
        let handfuls: Vec<&str> = game.split(";").collect();
        let mut i = 1;
        let mut game_map: HashMap<&str, i32> = HashMap::new();
        for hf in handfuls {
            i = i+1;
            // println!("hf: {}",hf);
            let sets: Vec<&str> = hf.split(",").collect();
            for set in sets {
                // println!("set: {}", set);
                let value_str: &str = find_substring(set,' ',' ').trim();
                let color:&str = find_suffix_from_prefix(set, value_str).trim();
                let value = str_to_i32(value_str);
                // print!("Value:{}", value);
                // println!(" Color:{}", color);
                match game_map.get_mut(color){
                    Some(existing_value) => {
                        if value > *existing_value{
                            *existing_value = value;
                        }
                    }
                    None => {
                        game_map.insert(color,value);
                    }
                }
            }
        }
        let result: i32 = game_map.values().product();
        return result;
    }



    pub fn solution(){
        let file = File::open("./src/inputs/input_day2.txt").expect("File not found");
        let reader = BufReader::new(file);
        let mut result: i32 = 0;
        let mut result2: i32 = 0;
        for line in reader.lines(){
            match line{
                Ok(line) => {
                    result += possible(&line);
                    result2 += cube_power(&line);
                }
                Err(err) => {
                    eprintln!("Error reading lines: {}",err);
                }
            }
        }
        println!("Solution: {}", result);
        println!("Solution2: {}", result2);
    }


}
