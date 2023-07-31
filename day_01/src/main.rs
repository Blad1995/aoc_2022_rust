use std::fs::read_to_string;


fn main() {
    let mut max_calories = 0;
    let mut elf_calories = 0;
    for line in read_to_string("src/input.txt").unwrap().lines(){
        if line == "" {
            if elf_calories > max_calories {
                max_calories = elf_calories;
            }
            elf_calories = 0;
            continue;
        }
        elf_calories += line.to_string().parse::<u32>().unwrap()
    }
    println!("{}", max_calories);
}
