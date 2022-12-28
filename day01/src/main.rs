//use std::env;
use std::fs;

fn main() {
	// read in the input file, separating each group of lines separated by an empty line

	let contents = fs::read_to_string("input-01.txt")
		.expect("Should have been able to read the input file");

	// make a struct that contains the group of numbers and their sum
	#[derive(Debug, Clone)]
	struct ElfLoad {
		elf_num: i32,
		calories: Vec<i32>,
		sum: i32,
	}

	impl ElfLoad {
		fn new() -> ElfLoad {
			ElfLoad {
				elf_num: 0,
				calories: Vec::new(),
				sum: 0,
			}
		}
	}

	// take in each group of lines, separated by a blank line, and add them to a vector
	let mut elven_loads: Vec<ElfLoad> = Vec::new();
	let mut elf_load: ElfLoad = ElfLoad::new();

	for line in contents.lines() {
		if line == "" {
			elf_load.sum = elf_load.calories.iter().sum();
			elven_loads.push(elf_load);
			elf_load = ElfLoad::new();
		} else {
			elf_load.calories.push(line.parse::<i32>().unwrap_or_default());
		}
	}
	// push the last load
	elf_load.sum = elf_load.calories.iter().sum();
	elven_loads.push(elf_load);

	// number the loads
	for (elf_load_num, each_elf_load) in (1_i32..).zip(elven_loads.iter_mut()) {
		each_elf_load.elf_num = elf_load_num;
	}

	// get the total sum of each group
	for elf_looooad in elven_loads.iter_mut() {
		println!("Group: {:?}", elf_looooad);
		println!("Sum of group: {}", elf_looooad.sum);
	}

	// get the max sum from the list
	let looooads = elven_loads.clone();
	let top_sum = looooads.iter().max_by_key(|x| x.sum).unwrap();

	// get the top 3 groups
	let mut looooooooooooads = elven_loads.clone();
	let mut top_three: Vec<ElfLoad> = Vec::new();
	for _ in 0..3 {
		let max = looooooooooooads.iter_mut().max_by_key(|x| x.sum).unwrap();
		top_three.push(max.clone());
		looooooooooooads.retain(|x| x.elf_num != max.elf_num);
	}

	println!("Max sum: {:?}", top_sum);
	println!("Top three: {:?}", top_three);
}