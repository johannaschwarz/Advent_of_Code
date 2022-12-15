use std::fs;


fn main() {
    let input = fs::read_to_string("./input/day8.txt").unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut visible_trees = 0;
    let mut current_line = 0;

    for line in input.lines() {
        grid.push(Vec::new());
        for num in line.chars() {
            grid[current_line].push(num.to_string().parse().unwrap());
        }
        current_line += 1;
    }

    let height = current_line;
    let width = grid[0].len();

    visible_trees += width * 2;
    visible_trees += (height - 2) * 2;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let tree = grid[i][j];
            // look left
            let left = &grid[i][0..j].to_vec();
            if *left.iter().max().unwrap() < tree {
                visible_trees += 1;
                continue;
            } 
            // look right
            let right = &grid[i][j + 1..width].to_vec();
            if *right.iter().max().unwrap() < tree {
                visible_trees += 1;
                continue;
            }

            let mut visible = true;
            // look up
            for k in 0..i {
                if grid[k][j] >= tree {
                    visible = false;
                    break;
                }
            }
            if visible {
                visible_trees += 1;
                continue;
            }
            visible = true;
            // look down
            for k in i + 1..width {
                if grid[k][j] >= tree {
                    visible = false;
                    break;
                }
            }
            if visible {
                visible_trees += 1;
            }
        }
    }

    println!("{:?}", visible_trees);
}