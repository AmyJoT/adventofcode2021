use std::fs;

fn main() {
    let contents = fs::read_to_string("./day2.txt").expect("Error reading file");
    let data = contents.split("\n").collect::<Vec<_>>();
    println!("{}", instructions(data.clone()));
    println!("{}", instructions2(data.clone()));
}

fn instructions(data: Vec<&str>) -> i32 {
    let mut h = 0;
    let mut v = 0;

    for instruction in data {
        let d: Vec<&str> = instruction.split(' ').collect();

        match d[0] {
            "forward" => h = h + d[1].parse::<i32>().unwrap(),
            "up" => v = v - d[1].parse::<i32>().unwrap(),
            "down" => v = v + d[1].parse::<i32>().unwrap(),
            _ => println!("{} is not a valid instruction", d[0]),
        }
    }
    h * v
}

fn instructions2(data: Vec<&str>) -> i32 {
    let mut h = 0;
    let mut v = 0;
    let mut a = 0;

    for instruction in data {
        let d: Vec<&str> = instruction.split(' ').collect();

        match d[0] {
            "forward" => {
                h = h + d[1].parse::<i32>().unwrap();
                v = v + (a * d[1].parse::<i32>().unwrap());
            },
            "up" => a = a - d[1].parse::<i32>().unwrap(),
            "down" => a = a + d[1].parse::<i32>().unwrap(),
            _ => println!("{} is not a valid instruction", d[0]),
        }
    }

    println!("{} {}", h, v);
    h * v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example() {

        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        assert_eq!(instructions(data), 150);
    }

    #[test]
    fn part_2_example() {

        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        assert_eq!(instructions2(data), 900);
    }
}
