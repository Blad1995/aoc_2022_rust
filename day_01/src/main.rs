use std::fs::read_to_string;

fn read_file(path: &str) -> Vec<String> {
    read_to_string(path).unwrap().lines().map(|s| s.to_string().clone()).collect()
}

fn get_elven_loads(lines: &Vec<String>) -> Vec<u32> {
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut elf: Vec<u32> = Vec::new();
    for line in lines {
        if line == "" {
            elves.push(elf);
            elf = Vec::new();
            continue;
        }
        elf.push(line.to_string().parse::<u32>().unwrap());
    }
    let summed = elves.iter().map(|vec| vec.iter().sum()).collect::<Vec<u32>>();
    summed
}

fn cound_sum_of_max_n(elves: &Vec<u32>, n: usize)  -> u32 {
    let mut top_n_elves: Vec<&u32> = elves[0..n].iter().collect();
    let mut nth_max = *top_n_elves.iter().min().expect("min not found");
    for line in elves[n..].iter() {
        if line > nth_max {
            top_n_elves.swap_remove(top_n_elves.iter().position(|&x| x == nth_max).unwrap());
            top_n_elves.push(line);
            nth_max = *top_n_elves.iter().min().expect("min not found");
        }
    }
    return top_n_elves.iter().map(|&e| e).sum();
}

fn main() {
    let lines = read_file("src/input.txt");
    let elves: Vec<u32> = get_elven_loads(&lines);
    let max_calories = cound_sum_of_max_n(&elves, 3);
    println!("{}", max_calories);
}
