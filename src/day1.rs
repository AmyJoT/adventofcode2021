use std::fs;

fn main() {
    let contents = fs::read_to_string("./day1.txt").expect("Error reading file");
    let data = contents.split("\n").collect::<Vec<_>>();

    println!("Part 1: {}", how_many_greater(data.clone()));
    println!("Part 2: {}", how_many_greater2(sums_of_threes(data.clone())));
}

fn how_many_greater(data: Vec<&str>) -> i32 {
    let mut count = 0;

    for (i, measurement) in data.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let current = measurement.parse::<i32>().unwrap();
        let previous = data[i - 1].parse::<i32>().unwrap();

        if current > previous {
            count = count + 1;
        }
    }

    count
}

fn how_many_greater2(data: Vec<i32>) -> i32 {
    let mut count = 0;

    for (i, measurement) in data.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if measurement > &data[i - 1] {
            count = count + 1;
        }
    }

    count
}

fn sums_of_threes(data: Vec<&str>) -> Vec<i32> {
    let mut vec = Vec::new();
    for (i, measurement) in data.iter().enumerate() {
        if i < data.len() - 2 {
            let first = measurement.parse::<i32>().unwrap();
            let second = data[i + 1].parse::<i32>().unwrap();
            let third = data[i + 2].parse::<i32>().unwrap();

            vec.push(first + second + third)
        }
    }

    vec
}