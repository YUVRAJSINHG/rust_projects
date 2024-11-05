use std::fs;
use regex::Regex;

fn main() {
    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    
    let input_text = fs::read_to_string("calibration_document.txt")
        .expect("should have been able to read the file");
    
    let mut final_vector: Vec<u32> = Vec::new();
    
    for line in input_text.lines() {
        let num_vector: Vec<&str> = re.find_iter(line).map(|mat|{
            let value = mat.as_str();
            match value {
                "one"   => "1",
                "two"   => "2",
                "three" => "3",
                "four"  => "4",
                "five"  => "5",
                "six"   => "6",
                "seven" => "7",
                "eight" => "8",
                "nine"  => "9",
                n       => n,
            }
        }).collect();
        let mut double = String::new();
        if num_vector.len() > 1 {
            double.push_str(num_vector[0]);
            double.push_str(num_vector[num_vector.len() -1]);
        }else{
            double.push_str(num_vector[0]);
            double.push_str(num_vector[0]);
        }
        final_vector.push(double.parse().unwrap())
    }
    let result: u32 = final_vector.iter().sum();
    println!("result of sum: {}", result);
}