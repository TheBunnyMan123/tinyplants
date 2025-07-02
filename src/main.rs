use core::f32;
use std::{collections::HashMap, fs};

use svg::{node::element::{path::Data, Group, Path}, Document};

const RADIAN_CONSTANT: f32 = f32::consts::PI / 180.0;

fn main() {
    let system = fs::read_to_string("system.txt").expect("Failed to read file 'systems.txt'");
    let mut rules: HashMap<char, String> = HashMap::new();
    let mut axiom = "".to_string();

    let mut iterations = 4;
    let mut angle: f32 = 30.0;
    let mut length: f32 = 30.0;

    let mut saved_positions: Vec<(f32, (f32, f32))> = Vec::new();
    let mut current_position: (f32, f32) = (0.0, 0.0);
    let mut current_angle: f32 = -90.0;
    
    let system_lines = system.lines();
    let mut i = 1;
    let count = system_lines.clone().count();
    for line in system_lines {
        if i == 1 {
            for option in line.split(";") {
                let (name, val) = option.trim().split_once(":").expect("Invalid format. Each config option must be name:val");

                match name {
                    "iter" => iterations = u32::from_str_radix(val, 10).expect("Iteration count must be a number"),
                    "angle" => angle = u32::from_str_radix(val, 10).expect("Angle must be a number") as f32,
                    "length" => length = u32::from_str_radix(val, 10).expect("Length must be a number") as f32,
                    _ => {}
                }
            }
        } else if i == count {
            axiom = line.to_string();
        } else {
            let (char, result) = line.split_once(":").expect("Invalid file format. Expected [char]:[result] (e.g. F>FF)");
            rules.insert(char.chars().next().expect("Invalid file format. Expected [char]:[result] (e.g. F>FF)"), result.to_string());
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

    let mut min_x: f32 = 0.0;
    let mut min_y: f32 = 0.0;
    let mut max_x: f32 = 0.0;
    let mut max_y: f32 = 0.0;

    let mut path_data = Data::new()
        .move_to((0.0, 0.0));

    for char in axiom.chars() {
        match char {
            'F' => {
                let new_x = current_position.0 + length * (current_angle * RADIAN_CONSTANT).cos();
                let new_y = current_position.1 + length * (current_angle * RADIAN_CONSTANT).sin();

                min_x = min_x.min(new_x.floor());
                min_y = min_y.min(new_y.floor());
                max_x = max_x.max(new_x.ceil());
                max_y = max_y.max(new_y.ceil());

                current_position = (new_x, new_y);

                path_data = path_data.line_to((new_x, new_y));
            },
            'f' => {
                let new_x = current_position.0 + length * (current_angle * RADIAN_CONSTANT).cos();
                let new_y = current_position.1 + length * (current_angle * RADIAN_CONSTANT).sin();

                min_x = min_x.min(new_x.floor());
                min_y = min_y.min(new_y.floor());
                max_x = max_x.max(new_x.ceil());
                max_y = max_y.max(new_y.ceil());
                
                current_position = (new_x, new_y);

                path_data = path_data.move_to((new_x, new_y));
            },
            '+' => current_angle += angle,
            '-' => current_angle -= angle,
            '[' => saved_positions.push((current_angle.clone(), current_position.clone())),
            ']' => {
                (current_angle, current_position) = saved_positions.pop().expect("Tried to load an unsaved position");
                path_data = path_data.move_to(current_position);
            },
            _ => {}
        }
    }
    
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", path_data);

    let group = Group::new()
        .set("transform", format!("translate({} {})", -min_x + 3.0, -min_y + 3.0))
        .add(path);

    let doc = Document::new()
        .add(group)
        .set("viewBox", (0, 0, max_x + min_x.abs() + 6.0, max_y + min_y.abs() + 6.0));

    svg::save("out.svg", &doc).expect("Failed to save SVG");
}
