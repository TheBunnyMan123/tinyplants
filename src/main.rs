use std::{collections::HashMap, fs};

fn main() {
    let system = fs::read_to_string("system.txt").expect("Failed to read file 'systems.txt'");
    let mut rules: HashMap<char, &str> = HashMap::new();
    let mut axiom = "".to_string();

    let mut iterations = 4;
    let mut angle = 30;
    let mut length = 30;

    let mut saved_positions: Vec<(isize, isize)> = Vec::new();
    let mut current_position: (isize, isize) = (0, 0);
    let mut current_angle: isize = -90;
    
    let system_lines = system.lines();
    let mut i = 1;
    let count = system_lines.clone().count();
    for line in system_lines {
        if i == 1 {
            for option in line.split(";") {
                let (name, val) = option.split_once(":").expect("Invalid format. Each config option must be name:val");

                match name {
                    "iter" => iterations = u32::from_str_radix(val, 10).expect("Iteration count must be a number"),
                    "angle" => angle = u32::from_str_radix(val, 10).expect("Angle must be a number"),
                    "length" => length = u32::from_str_radix(val, 10).expect("Length must be a number"),
                    _ => {}
                }
            }
        } else if i == count {
            axiom = line.to_string();
        } else {
            let (char, result) = line.split_once(":").expect("Invalid file format. Expected [char]:[result] (e.g. F>FF)");
            rules.insert(char.chars().next().expect("Invalid file format. Expected [char]:[result] (e.g. F>FF)"), result);
        }

        i += 1;
    }

    for _ in 0..iterations {
        let mut axiom_replacement: String = "".to_string();

        for char in axiom.chars() {
            match rules.get(&char) {
                Some(rule) => axiom_replacement += rule,
                None => axiom_replacement += char.to_string().as_str()
            }
        }

        axiom = axiom_replacement;
    }

    println!("{}", axiom);
}
