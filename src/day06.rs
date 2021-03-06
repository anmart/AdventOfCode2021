#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut fishies: [u32;9] = [0;9];
	
	// Warning: side effects....it's pretty though
	lines.trim().split(",").for_each(|s| fishies[s.parse::<usize>().unwrap()]+=1);	
	
	for i in 0..80{
		let birthing_fishies = fishies[i%9];
		fishies[(i+7)%9] += birthing_fishies;
		//println!("{:?}, 0 was {}",fishies, i%9);
	}
	return fishies.iter().sum::<u32>().to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	
	let mut fishies: [u64;9] = [0;9];
	
	// Warning: side effects....it's pretty though
	lines.trim().split(",").for_each(|s| fishies[s.parse::<usize>().unwrap()]+=1);	
	
	// Warning: side effects....it's pretty though (again)
	// this is so stupid, literally just use a for loop
	(0..256).for_each(|i| fishies[(i+7)%9] += fishies[i%9]);

	return fishies.iter().sum::<u64>().to_string();
}
