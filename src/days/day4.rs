use std::iter::repeat;

pub fn solve_one(mut input:String) -> i32 {
    let mut sum = 0;
    let length = input.lines().clone().next().unwrap_or("").chars().count();
    let height = input.lines().clone().count();
    let search_word_length=4;
    for _ in 0..search_word_length {
        input.push_str(&(repeat('.').take(height).collect::<String>() + "\n"));
    }
    let padded_input_lines = input.lines().map(|l| l.to_owned() + &repeat('.').take(search_word_length).collect::<String>());
    let veced_padded_input: Vec<Vec<char>> = padded_input_lines.map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    for x in 0..length {
        for y in 0..height {
            let arrangements: Vec<String> = vec![
                veced_padded_input[x][y..y+search_word_length].iter().collect(),
                veced_padded_input[x..x+search_word_length].iter().map(|l| l[y]).collect(),
                (0..search_word_length).map(|i| veced_padded_input[x+i][y+i]).collect(),
                (0..search_word_length).map(|i| veced_padded_input[x+i][y+search_word_length-1-i]).collect()];
            for arrangement in arrangements {
                if let "XMAS" = &arrangement[..] {
                    sum += 1;
                }
                if let "SAMX" = &arrangement[..] {
                    sum += 1;
                }
            }
        }
    }
    sum
}


pub fn solve_two(mut input:String) -> i32 {
    let mut sum = 0;
    let length = input.lines().clone().next().unwrap_or("").chars().count();
    let height = input.lines().clone().count();
    let search_word_length=3;
    for _ in 0..search_word_length {
        input.push_str(&(repeat('.').take(height).collect::<String>() + "\n"));
    }
    let padded_input_lines = input.lines().map(|l| l.to_owned() + &repeat('.').take(search_word_length).collect::<String>());
    let veced_padded_input: Vec<Vec<char>> = padded_input_lines.map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    for x in 0..length {
        for y in 0..height {
            let arrangements: Vec<String> = vec![
                (0..search_word_length).map(|i| veced_padded_input[x+i][y+i]).collect::<String>(),
                (0..search_word_length).map(|i| veced_padded_input[x+i][y+search_word_length-1-i]).collect::<String>()];
            if arrangements.iter().all(|v|v=="MAS" ||v=="SAM") {
                sum+=1;
            }
        }
    }
    sum
}
