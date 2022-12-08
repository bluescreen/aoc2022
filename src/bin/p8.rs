use std::collections::HashSet;
use aoc2022::util::read_line;

fn main() {
    let lines = read_line("./input/p8.txt").unwrap();

    let (total, max) = solve(&lines);
    println!("Part 1: {}", total);
    println!("Part 2: {}", max);
}

fn solve(lines: &str) -> (usize, usize) {
    let grid = parse_to_grid(lines);
    let visible = find_visible_trees(&grid);
    let count_visibles = visible.len();
    let max = calculate_max_scenic_score(grid);

    (count_visibles, max)
}

fn find_visible_trees(grid: &Vec<Vec<i32>>) -> HashSet<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..height {
        // left
        let mut value = -1;
        for x in 0..width {
            if grid[y][x] > value {
                value = grid[y][x];
                visible.insert((x, y));
            }
        }

        // right
        let mut value = -1;
        for x in (0..width).rev() {
            if grid[y][x] > value {
                value = grid[y][x];
                visible.insert((x, y));
            }
        }
    }
    for x in 0..width {
        // left
        let mut value = -1;
        for y in 0..height {
            if grid[y][x] > value {
                value = grid[y][x];
                visible.insert((x, y));
            }
        }
        // right
        let mut value = -1;
        for y in (0..height).rev() {
            if grid[y][x] > value {
                value = grid[y][x];
                visible.insert((x, y));
            }
        }
    }
    visible
}

fn calculate_max_scenic_score(grid: Vec<Vec<i32>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut max = 0;

    for y in 0..height {
        for x in 0..width {
            max = std::cmp::max(max, scenic_score_for_tree(x, y, &grid));
        }
    }
    max
}

fn scenic_score_for_tree(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let tree_height = grid[y][x];

    //  right
    let mut right_score = 0;
    for x in x + 1..width {
        right_score += 1;
        if grid[y][x] >= tree_height {
            break;
        }
    }
    //  left
    let mut left_score = 0;
    for x in (0..x).rev() {
        left_score += 1;
        if grid[y][x] >= tree_height {
            break;
        }
    }

    let mut down_score = 0;
    for y in y + 1..height {
        down_score += 1;
        if grid[y][x] >= tree_height {
            break;
        }
    }
    //  up
    let mut up_score = 0;
    for y in (0..y).rev() {
        up_score += 1;
        if grid[y][x] >= tree_height {
            break;
        }
    }
    right_score * left_score * up_score * down_score
}

fn parse_to_grid(lines: &str) -> Vec<Vec<i32>> {
    let grid: Vec<Vec<i32>> = lines
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| { num.to_string().parse::<i32>().unwrap() })
                .collect()
        })
        .collect();
    grid
}