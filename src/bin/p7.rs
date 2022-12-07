use aoc2022::util::read_line;

fn main() {
    let lines = read_line("./input/p7.txt").unwrap();

    let (total, to_delete) = get_directory_size(&lines);
    println!("Part 1 {}", total);
    println!("Part 2 {}", to_delete);
}

fn get_directory_size(commands: &str) -> (u32, u32) {
    let report_amount = 100_000;
    let mut report_filesize = 0;
    let mut total_filesize = 0;
    let mut stack: Vec<u32> = vec![0];
    let mut directory_list = vec![0];

    for x in commands.lines().skip(1) {
        let parts: Vec<&str> = x.split(" ").collect();
        if parts[0] == "dir" || parts[1] == "ls" {
            continue;
        }
        if parts[0] == "$" && parts[1] == "cd" {
            let dir = parts[2];
            if dir == ".." {
                let amount = stack.pop().unwrap();
                if amount <= report_amount {
                    report_filesize += amount;
                }
                *stack.last_mut().unwrap() += amount;
                directory_list.push(amount);
            } else {
                stack.push(0);
            }
        } else {
            let file_size = parts[0].parse::<u32>().expect("not a number");
            *stack.last_mut().unwrap() += file_size;
            total_filesize += file_size;
        }
    }

    walk_backwards(stack, &mut directory_list);
    let to_delete = find_smallest_directory_to_delete(directory_list, total_filesize);

    (report_filesize, to_delete)
}

fn walk_backwards(mut stack: Vec<u32>, directory_list: &mut Vec<u32>) {
    while stack.len() > 0 {
        let amount = stack.pop().unwrap();
        directory_list.push(amount);
        if stack.len() > 0 {
            *stack.last_mut().unwrap() += amount;
            println!("{}", *stack.last_mut().unwrap());
        }
    }
}

fn find_smallest_directory_to_delete(directory_list: Vec<u32>, total_filesize: u32) -> u32 {
    let total_space = 70_000_000;
    let space_to_delete = 30_000_000;
    let space_required = space_to_delete - (total_space - total_filesize);

    directory_list
        .into_iter()
        .filter(move |size| size >= &space_required)
        .min()
        .unwrap()
}