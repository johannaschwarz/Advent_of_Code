use std::{collections::HashMap, fs};

fn main() {
    let console_output = fs::read_to_string("./input/day7.txt").unwrap();
    let mut lines = console_output.lines();
    lines.next();

    let mut current_directories: HashMap<String, Dir> = HashMap::new();
    let mut file_system: HashMap<String, Dir> = HashMap::new();

    let mut current_path = String::from("/");
    current_directories.insert(current_path.clone(), Dir::new());

    for line in lines {
        if line.starts_with('$') {
            let command = &line[2..=3];

            if command == "cd" {
                let name = &line[5..].to_string();

                if name == ".." {
                    let (path, dir) = current_directories.remove_entry(&current_path).unwrap();
                    file_system.insert(path, dir);

                    current_path.pop();
                    let (path, _two) = current_path.rsplit_once('/').unwrap();
                    current_path = path.to_owned() + "/";
                } else {
                    current_path.push_str(&(name.to_owned() + "/"));

                    let (path, dir) = file_system.remove_entry(&current_path).unwrap();
                    current_directories.insert(path, dir);
                }
            }
        } else if line.starts_with("dir") {
            let name = &line[4..].to_string();
            let path = current_path.clone() + name + "/";
            file_system.insert(path, Dir::new());
        } else {
            let mut line = line.split_ascii_whitespace();
            let size = line.next().unwrap().parse::<i32>().unwrap();

            current_directories
                .iter_mut()
                .for_each(|(_path, dir)| dir.size += size);
        }
    }

    while !current_path.is_empty() {
        let (path, dir) = current_directories.remove_entry(&current_path).unwrap();
        file_system.insert(path, dir);
        current_path.pop();
        while !current_path.ends_with('/') && !current_path.is_empty() {
            current_path.pop();
        }
    }

    let used_space = file_system.get("/").unwrap();
    let free_space = 70000000 - used_space.size;
    let needed_space = 30000000 - free_space;

    let min = file_system.iter().map(|(_path, dir)| dir.size).filter(|size| *size >= needed_space).min().unwrap();
    println!("Min: {}", min);
}
struct Dir {
    size: i32,
}

impl Dir {
    fn new() -> Dir {
        Dir { size: 0 }
    }
}
