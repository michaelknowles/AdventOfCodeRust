use super::util;

pub fn get_input() -> Vec<i32> {
    util::read_file("./resources/2019-day1.txt")
        .iter()
        .map(|x| x.parse::<i32>().expect("Couldn't convert to int"))
        .collect()
}

pub fn day1() {
    
    println!("Day 1");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn get_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

pub fn part1(input: &Vec<i32>) -> i32 {
    input.iter().fold(0, |acc, x| acc + get_fuel(*x))
}

fn get_fuel_rec(mut mass: i32) -> i32 {
    let mut fuel = 0;
    while mass >= 0 {
        mass = get_fuel(mass);
        fuel += if mass >= 0 {mass} else {0};
    }
    fuel
}

pub fn part2(input: &Vec<i32>) -> i32 {
    input.iter().fold(0, |acc, x| acc + get_fuel_rec(*x))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1 tests
    #[test]
    fn fuel_12() {
        assert_eq!(get_fuel(12), 2);
    }

    #[test]
    fn fuel_14() {
        assert_eq!(get_fuel(14), 2);
    }

    #[test]
    fn fuel_1969() {
        assert_eq!(get_fuel(1_969), 654);
    }

    #[test]
    fn fuel_100756() {
        assert_eq!(get_fuel(100_756), 33_583);
    }

    // Part 2 tests
    #[test]
    fn fuel_rec_14() {
        assert_eq!(get_fuel_rec(14), 2);
    }

    #[test]
    fn fuel_rec_1969() {
        assert_eq!(get_fuel_rec(1_969), 966);
    }

    #[test]
    fn fuel_rec_100756() {
        assert_eq!(get_fuel_rec(100_756), 50_346);
    }
}