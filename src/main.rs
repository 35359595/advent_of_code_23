#![allow(dead_code, unused_assignments, unused_imports, unused_variables)]
use core::panic;
use rayon::prelude::*;
use std::collections::btree_map::Range;
use std::collections::vec_deque::VecDeque;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_lines(day: u8) -> impl Iterator<Item = Result<String, impl std::error::Error>> {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = format!("{}/inputs/{}", cargo_dir, day);
    let Ok(file) = File::open(&dir) else {
        panic!("File for day {day} not found at {dir}");
    };
    BufReader::new(file).lines().into_iter()
}

fn day_1() {
    let result_pt1 = read_lines(1)
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|l| {
            let digits = l
                .chars()
                .into_iter()
                .filter_map(|c| c.to_digit(10).ok_or(0).ok())
                .collect::<Vec<u32>>();
            digits[0] * 10 + digits.last().unwrap()
        })
        .sum::<u32>();
    println!("Result part 1 for day 1: {}", result_pt1);
    // Part 2
    let regexp = "(?:one|two|three|four|five|six|seven|eight|nine|[1-9])";
    let from_word = |w: &str| -> u32 {
        match w {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => w.parse().unwrap(),
        }
    };
    let regex = fancy_regex::RegexBuilder::new(regexp)
        .backtrack_limit(usize::MAX)
        .build()
        .unwrap();
    let result_pt2 = read_lines(1)
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|l| {
            let mut digits: Vec<u32> = vec![];
            let mut start = 0;
            // cumbersome collapsing regex
            while let Ok(Some(m)) = regex.captures_from_pos(&l, start) {
                let matched = m.get(0).unwrap().as_str();
                digits.push(from_word(matched));
                start = m.get(0).unwrap().start() + 1;
            }
            digits[0] * 10 + digits.last().unwrap()
        })
        .sum::<u32>();
    println!("Result part 2 for day 1: {}", result_pt2);
}

fn day_2() {
    let inputs = read_lines(2);
    let fits = |set: &str| -> bool {
        let stones = set.split(',');
        for stone in stones {
            let pair: Vec<&str> = stone.trim().split(' ').collect();
            //println!("{pair:?}");
            let count: u32 = pair[0].parse().unwrap();
            match pair[1] {
                "blue" if count > 14 => {
                    //println!("false blue");
                    return false;
                }
                "green" if count > 13 => {
                    //println!("false green");
                    return false;
                }
                "red" if count > 12 => {
                    //println!("false red");
                    return false;
                }
                _ => (),
            }
        }
        //println!("true");
        true
    };
    let res_part1 = inputs
        .into_iter()
        .filter_map(|s| s.ok())
        .enumerate()
        .filter(|(n, full)| full.split(';').all(fits))
        .fold(0, |acc, (num, _)| acc + num + 1);
    println!("Result for day 2: {}", res_part1);
    // # part 2
    let inputs = read_lines(2);
    let min_required = |full: &str| -> u32 {
        let mut all = HashMap::new();
        all.insert("blue", 0);
        all.insert("green", 0);
        all.insert("red", 0);
        for set in full.split(';') {
            let stones = set.split(',');
            for stone in stones {
                let pair: Vec<&str> = stone.trim().split(' ').collect();
                let num: u32 = pair[0].parse().unwrap();
                if *all.get(pair[1]).unwrap() < num {
                    all.insert(pair[1], num);
                }
            }
        }
        let mut blue = all.get("blue").unwrap();
        let mut red = all.get("red").unwrap();
        let mut green = all.get("green").unwrap();
        blue = if blue == &0 { &1 } else { blue };
        red = if red == &0 { &1 } else { red };
        green = if green == &0 { &1 } else { green };
        //println!("b {blue}; r {red}; g {green}");
        red * blue * green
    };
    let res_part2 = inputs
        .into_iter()
        .filter_map(|s| s.ok())
        .fold(0, |acc, full| acc + min_required(&full));
    println!("part 2: {res_part2}");
}

fn day_3() {
    let inputs = read_lines(3);
    fn consider_number_neighbors(
        board: &Vec<String>,
        gear_nums: &mut HashMap<(usize, usize), Vec<i32>>,
        start_y: usize,
        start_x: usize,
        end_y: usize,
        end_x: usize,
        num: i32,
    ) -> bool {
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                if y < board.len() && x < board[y].len() {
                    if !board[y].chars().nth(x).unwrap().is_numeric()
                        && board[y].chars().nth(x).unwrap() != '.'
                    {
                        if board[y].chars().nth(x).unwrap() == '*' {
                            gear_nums.entry((y, x)).or_insert(vec![]).push(num);
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
    let mut total = 0;
    let mut board = Vec::new();
    let mut gear_nums = HashMap::new();

    let num_pattern = fancy_regex::Regex::new(r"\d+").unwrap();

    for line in inputs.filter_map(|l| l.ok()) {
        board.push(line.trim().to_string());
    }

    for (row_num, line) in board.iter().enumerate() {
        for mat in num_pattern.find_iter(line).filter_map(|m| m.ok()) {
            if consider_number_neighbors(
                &board,
                &mut gear_nums,
                row_num.saturating_sub(1),
                mat.start().saturating_sub(1),
                row_num + 1,
                mat.end(),
                mat.as_str().parse().unwrap(),
            ) {
                total += mat.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("part 1: {}", total);

    let mut rat_total = 0;
    for (_, v) in gear_nums.iter() {
        if v.len() == 2 {
            rat_total += v[0] * v[1];
        }
    }

    println!("part 2: {}", rat_total);
}

fn day_4() {
    let inputs: Vec<String> = read_lines(4).filter_map(|s| s.ok()).collect();
    let res_part1: u32 = inputs
        .into_iter()
        .map(|full| {
            let both: Vec<String> = full.trim().split('|').map(|s| s.to_string()).collect();
            let winning: Vec<u32> = both[0]
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            let res: u32 = both[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .fold(0, |acc, v| {
                    let parsed = v.trim().parse().unwrap();
                    if winning.contains(&parsed) {
                        if acc == 0 {
                            1
                        } else {
                            acc + acc
                        }
                    } else {
                        acc
                    }
                });
            res
        })
        .sum();
    println!("Result for day 4: {}", res_part1);
    // #Part 2
    let inputs: Vec<String> = read_lines(4).filter_map(|s| s.ok()).collect();
    println!("Result for day 4: {}", day_2_part_2(inputs));
}

fn day_2_part_2(inputs: impl IntoIterator<Item = String> + Clone) -> u32 {
    let mut per_card = vec![0u32; inputs.clone().into_iter().count()];
    inputs
        .into_iter()
        .enumerate()
        .map(|(id, full)| {
            let both: Vec<String> = full.trim().split('|').map(|s| s.to_string()).collect();
            let winning: Vec<u32> = both[0]
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            let res: u32 = both[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .fold(0, |acc, v| {
                    let parsed = v.trim().parse().unwrap();
                    if winning.contains(&parsed) {
                        acc + 1
                    } else {
                        acc
                    }
                });
            for i in id + 1..=id + res as usize {
                per_card[i] += 1 + per_card[id];
            }
            per_card[id] += 1;
        })
        .for_each(drop);
    per_card.into_iter().sum()
}

#[test]
fn day_4_part_2_test() {
    let set: [String; 6] = [
        "41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];
    assert_eq!(30, day_2_part_2(set));
}

fn day_5() {
    let inputs = read_lines(5);
    let mut inputs = inputs.filter_map(|i| i.ok()).into_iter();
    let mut seeds = vec![];
    let mut seed_to_soil = vec![];
    let mut soil_to_fertilizer = vec![];
    let mut fertilizer_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temperature = vec![];
    let mut temperature_to_humidity = vec![];
    let mut humidity_to_location = vec![];

    while let Some(l) = inputs.next() {
        if l.trim().eq("seeds:") {
            while let Some(l) = inputs.next() {
                if l.trim().eq("seed-to-soil:") {
                    break;
                }
                seeds.extend(l.split(' ').map(|s| s.parse::<u64>().unwrap()));
            }
            while let Some(l) = inputs.next() {
                if l.trim().eq("soil-to-fertilizer:") {
                    break;
                }
                seed_to_soil.extend(
                    l.split(' ')
                        .collect::<Vec<&str>>()
                        .chunks(3)
                        .map(|d| {
                            S::new(
                                d[1].parse().unwrap(),
                                d[0].parse().unwrap(),
                                d[2].parse().unwrap(),
                            )
                        })
                        .collect::<Vec<S>>(),
                );
            }
            while let Some(l) = inputs.next() {
                if l.trim().eq("fertilizer-to-water:") {
                    break;
                }
                soil_to_fertilizer.extend(
                    l.split(' ')
                        .collect::<Vec<&str>>()
                        .chunks(3)
                        .map(|d| {
                            S::new(
                                d[1].parse().unwrap(),
                                d[0].parse().unwrap(),
                                d[2].parse().unwrap(),
                            )
                        })
                        .collect::<Vec<S>>(),
                );
            }
            while let Some(l) = inputs.next() {
                if l.trim().eq("water-to-light:") {
                    break;
                }
                fertilizer_to_water.extend(
                    l.split(' ')
                        .collect::<Vec<&str>>()
                        .chunks(3)
                        .map(|d| {
                            S::new(
                                d[1].parse().unwrap(),
                                d[0].parse().unwrap(),
                                d[2].parse().unwrap(),
                            )
                        })
                        .collect::<Vec<S>>(),
                );
            }
            while let Some(l) = inputs.next() {
                if l.trim().eq("light-to-temperature:") {
                    break;
                }
                water_to_light.extend(
                    l.split(' ')
                        .collect::<Vec<&str>>()
                        .chunks(3)
                        .map(|d| {
                            S::new(
                                d[1].parse().unwrap(),
                                d[0].parse().unwrap(),
                                d[2].parse().unwrap(),
                            )
                        })
                        .collect::<Vec<S>>(),
                );
            }
            while let Some(l) = inputs.next() {
                if l.trim().eq("temperature-to-humidity:") {
                    break;
                }
                light_to_temperature.extend(
                    l.split(' ')
                        .collect::<Vec<&str>>()
                        .chunks(3)
                        .map(|d| {
                            S::new(
                                d[1].parse().unwrap(),
                                d[0].parse().unwrap(),
                                d[2].parse().unwrap(),
                            )
                        })
                        .collect::<Vec<S>>(),
                );
            }
            while let Some(l) = inputs.next() {
                if l.trim().eq("humidity-to-location:") {
                    break;
                }
                temperature_to_humidity.extend(
                    l.split(' ')
                        .collect::<Vec<&str>>()
                        .chunks(3)
                        .map(|d| {
                            S::new(
                                d[1].parse().unwrap(),
                                d[0].parse().unwrap(),
                                d[2].parse().unwrap(),
                            )
                        })
                        .collect::<Vec<S>>(),
                );
            }
            while let Some(l) = inputs.next() {
                humidity_to_location.extend(
                    l.split(' ')
                        .collect::<Vec<&str>>()
                        .chunks(3)
                        .map(|d| {
                            S::new(
                                d[1].parse().unwrap(),
                                d[0].parse().unwrap(),
                                d[2].parse().unwrap(),
                            )
                        })
                        .collect::<Vec<S>>(),
                );
            }
        }
    }

    let res_pt_1 = seeds
        .iter()
        .map(|s| {
            find(
                &humidity_to_location,
                find(
                    &temperature_to_humidity,
                    find(
                        &light_to_temperature,
                        find(
                            &water_to_light,
                            find(
                                &fertilizer_to_water,
                                find(&soil_to_fertilizer, find(&seed_to_soil, *s)),
                            ),
                        ),
                    ),
                ),
            )
        })
        .min()
        .unwrap();

    println!("Result for day 5: {}", res_pt_1);
}

fn find(set: impl AsRef<[S]>, what: u64) -> u64 {
    let Some(found) = set.as_ref().par_iter().find_first(|v| v.find(what) != what) else {
        return what;
    };
    found.find(what)
}

struct S {
    source: u64,
    dest: u64,
    offset: u64,
}

impl S {
    fn new(min: u64, max: u64, set: u64) -> Self {
        S {
            source: min,
            dest: max,
            offset: set,
        }
    }

    fn find(&self, what: u64) -> u64 {
        // in range
        let max = self.source + self.offset;
        if what >= self.source && what <= max {
            let dif: i128 = self.source as i128 - self.dest as i128;
            (max as i128 - self.dest as i128 + dif) as u64
        } else {
            what
        } // same
    }
}

#[test]
fn day_5_test() {
    let seeds = [79, 14, 55, 13];

    let seed_to_soil = vec![S::new(98, 50, 2), S::new(50, 52, 48)];
    let soil_to_fertilizer = vec![S::new(15, 0, 37), S::new(52, 37, 2), S::new(0, 39, 15)];
    let fertilizer_to_water = vec![
        S::new(53, 49, 8),
        S::new(11, 0, 42),
        S::new(0, 42, 7),
        S::new(7, 57, 4),
    ];
    let water_to_light = vec![S::new(18, 88, 7), S::new(25, 18, 70)];

    let light_to_temperature = vec![S::new(77, 45, 23), S::new(45, 81, 19), S::new(64, 68, 13)];

    let temperature_to_humidity = vec![S::new(69, 0, 1), S::new(0, 1, 69)];

    let humidity_to_location = vec![S::new(56, 60, 37), S::new(93, 56, 4)];
    for i in seeds {
        println!("for {i}");
        let sts = find(&seed_to_soil, i);
        println!("{i} > {sts}");
        let stf = find(&soil_to_fertilizer, sts);
        println!("{sts} > {stf}");
        let ftw = find(&fertilizer_to_water, stf);
        println!("{stf} > {ftw}");
        let wtl = find(&water_to_light, ftw);
        println!("{ftw} > {wtl}");
        let ltt = find(&light_to_temperature, wtl);
        println!("{wtl} > {ltt}");
        let tth = find(&temperature_to_humidity, ltt);
        println!("{ltt} > {tth}");
        let htl = find(&humidity_to_location, tth);
        println!("{tth} > {htl}");
    }
    let res_pt_1 = seeds.iter().map(|s| {
        find(
            &humidity_to_location,
            find(
                &temperature_to_humidity,
                find(
                    &light_to_temperature,
                    find(
                        &water_to_light,
                        find(
                            &fertilizer_to_water,
                            find(&soil_to_fertilizer, find(&seed_to_soil, *s)),
                        ),
                    ),
                ),
            ),
        )
    });
    println!("{res_pt_1:?}");
    assert_eq!(res_pt_1.min().unwrap(), 35);
}

fn day_6() {
    println!("Result for day 6: {}", 0);
    let inputs = read_lines(6);
}

fn day_7() {
    println!("Result for day 7: {}", 0);
    let inputs = read_lines(7);
}

fn day_8() {
    println!("Result for day 8: {}", 0);
    let inputs = read_lines(8);
}

fn day_9() {
    println!("Result for day 9: {}", 0);
    let inputs = read_lines(9);
}

fn day_10() {
    println!("Result for day 10: {}", 0);
    let inputs = read_lines(10);
}

fn day_11() {
    println!("Result for day 11: {}", 0);
    let inputs = read_lines(11);
}

fn day_12() {
    println!("Result for day 12: {}", 0);
    let inputs = read_lines(12);
}

fn day_13() {
    println!("Result for day 13: {}", 0);
    let inputs = read_lines(13);
}

fn day_14() {
    println!("Result for day 14: {}", 0);
    let inputs = read_lines(14);
}

fn day_15() {
    println!("Result for day 15: {}", 0);
    let inputs = read_lines(15);
}

fn day_16() {
    println!("Result for day 16: {}", 0);
    let inputs = read_lines(16);
}

fn day_17() {
    println!("Result for day 17: {}", 0);
    let inputs = read_lines(17);
}

fn day_18() {
    println!("Result for day 18: {}", 0);
    let inputs = read_lines(18);
}

fn day_19() {
    println!("Result for day 19: {}", 0);
    let inputs = read_lines(19);
}

fn day_20() {
    println!("Result for day 20: {}", 0);
    let inputs = read_lines(20);
}

fn day_21() {
    println!("Result for day 21: {}", 0);
    let inputs = read_lines(21);
}

fn day_22() {
    println!("Result for day 22: {}", 0);
    let inputs = read_lines(22);
}

fn day_23() {
    println!("Result for day 23: {}", 0);
    let inputs = read_lines(23);
}

fn day_24() {
    println!("Result for day 24: {}", 0);
    let inputs = read_lines(24);
}

fn main() {
    let args: Vec<String> = args().collect();
    match args[1].parse() {
        Ok(day) if (1u8..=24).contains(&day) => {
            println!("Running impl of day {} challenge..", { day });
            match day {
                1 => day_1(),
                2 => day_2(),
                3 => day_3(),
                4 => day_4(),
                5 => day_5(),
                6 => day_6(),
                7 => day_7(),
                8 => day_8(),
                9 => day_9(),
                10 => day_10(),
                11 => day_11(),
                12 => day_12(),
                13 => day_13(),
                14 => day_14(),
                15 => day_15(),
                16 => day_16(),
                17 => day_17(),
                18 => day_18(),
                19 => day_19(),
                20 => day_20(),
                21 => day_21(),
                22 => day_22(),
                23 => day_23(),
                24 => day_24(),
                _ => unreachable!(),
            }
        }
        _ => {
            println!("Provide day argument between 1 and 24");
        }
    }
}

#[test]
fn all_files_are_ok_open_test() {
    for day in 1..=24 {
        // no panicing here
        drop(read_lines(day));
    }
}
