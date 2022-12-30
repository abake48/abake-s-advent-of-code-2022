/*
* Calculate greatest num of calories
* Read line -> validate input (is_number?) -> add to sum
* if new line/invalid input, add sum to a list
* if EOF, compare sums, output largest total.
*/
//

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::Iterator;

fn main() {
    // Define variables
    let mut cal_sum = 0;
    let mut top_three = [0, 0, 0];
    let calorie_sheet = File::open("data/day1_input.txt").expect("Error Reading File");
    let calorie_sheet = BufReader::new(calorie_sheet);

    for calories in calorie_sheet.lines() {
        match calories.unwrap().parse::<u32>() {
            Ok(cals) => {
                cal_sum += cals;
            }
            Err(_e) => {
                match cal_sum {
                    one if one > top_three[0] => {
                        top_three[2] = top_three[1];
                        top_three[1] = top_three[0];
                        top_three[0] = cal_sum;
                    }
                    two if two > top_three[1] => {
                        top_three[2] = top_three[1];
                        top_three[1] = cal_sum;
                    }
                    three if three > top_three[2] => {
                        top_three[2] = cal_sum;
                    }
                    _ => {}
                }
                println!(
                    "Top 3 => 1.) {} 2.) {} 3.) {}",
                    top_three[0], top_three[1], top_three[2]
                );
                cal_sum = 0;
            }
        }
    }

    println!(
        "Sum of top three calories:{}",
        top_three.iter().sum::<u32>()
    );
}
