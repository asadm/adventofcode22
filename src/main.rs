use std::fs;
use std::env;
fn main() {
    let file_contents = fs::read_to_string("src/input.txt")
      .expect("error loading file");

    let lines = file_contents.split("\n");

    let mut currentElvCals = 0;
    let mut allElves = vec![];
    let mut maxElvCals = 0;
    for line in lines {
      if (line == ""){
        let idx = allElves.partition_point(|&x| x > currentElvCals);
        allElves.insert(idx, currentElvCals);
        currentElvCals = 0;
      }
      else{
        currentElvCals = currentElvCals + line.parse::<i32>().unwrap();;
      }

      if (currentElvCals > maxElvCals) {
        maxElvCals = currentElvCals;
      }
    }
    let firstThreeSum = allElves[0] + allElves[1] + allElves[2];
    // println!("{:?}", allElves);

    println!("Part1: {maxElvCals}"); // PART1
    println!("Part2: {firstThreeSum}");  // PART2
}
