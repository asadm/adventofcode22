use std::fs;
use std::env;
fn main() {
    let file_contents = fs::read_to_string("src/input.txt")
      .expect("error loading file");

    let lines = file_contents.split("\n");

    let mut currentElvCals = 0;
    let mut maxElvCals = 0;
    for line in lines {
      if (line == ""){
        currentElvCals = 0;
      }
      else{
        currentElvCals =currentElvCals + line.parse::<i32>().unwrap();;
      }

      if (currentElvCals > maxElvCals ) {
        maxElvCals = currentElvCals;
      }
    }
    println!("\n{maxElvCals}");
}
