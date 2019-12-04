use adventofcode::geometry::Point;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let file = File::open("input/2019-day3.txt").expect("Input file could not be opened");

    let wires: Vec<Vec<Point<i32>>> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read from buffer"))
        .map(|line| wire_points(&line))
        .collect();

    let wire1 = &wires[0];
    let wire2 = &wires[1];
    let mut intersect: Vec<Point<i32>> = vec![];

    for wire1_point in wire1 {
        for wire2_point in wire2 {
            if wire1_point == wire2_point {
                intersect.push(wire1_point.clone());
            }
        }
    }

    let origin = Point::new(0, 0);
    let mut selected: Option<Point<i32>> = None;
    let mut shortest_distance = std::i32::MAX;

    for intersect_point in intersect {
        let distance = origin.manhattan_distance(&intersect_point);

        if distance < shortest_distance {
            selected = Some(intersect_point.clone());
            shortest_distance = distance;
        }
    }

    match selected {
        None => process::exit(1),
        Some(_) => println!("{:?}", shortest_distance),
    }
}

#[derive(Debug)]
enum Travel {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn wire_points(wire_travel_codes: &str) -> Vec<Point<i32>> {
    let travels: Vec<Travel> = wire_travel_codes
        .trim()
        .split(",")
        .map(parse_travel)
        .collect();

    let mut result: Vec<Point<i32>> = vec![];
    let mut current_point = Point::new(0, 0);

    for travel in travels {
        let mut travel_points = points(&current_point, &travel);
        current_point = travel_points[travel_points.len() - 1];
        result.append(&mut travel_points);
    }

    result
}

fn parse_travel(travel_code: &str) -> Travel {
    match &travel_code[0..1] {
        "U" => Travel::Up(parse_travel_distance(travel_code)),
        "D" => Travel::Down(parse_travel_distance(travel_code)),
        "L" => Travel::Left(parse_travel_distance(travel_code)),
        "R" => Travel::Right(parse_travel_distance(travel_code)),
        _ => panic!("Invalid travel code {:?}", travel_code),
    }
}

fn parse_travel_distance(travel_code: &str) -> i32 {
    travel_code[1..]
        .parse()
        .expect("Could not parse travel distance")
}

fn points(from: &Point<i32>, travel: &Travel) -> Vec<Point<i32>> {
    let mut result: Vec<Point<i32>> = vec![];

    match travel {
        Travel::Up(distance) => {
            for i in from.y() + 1..from.y() + distance + 1 {
                result.push(Point::new(from.x(), i));
            }
        }
        Travel::Down(distance) => {
            for i in (from.y() - distance..from.y()).rev() {
                result.push(Point::new(from.x(), i));
            }
        }
        Travel::Right(distance) => {
            for i in from.x() + 1..from.x() + distance + 1 {
                result.push(Point::new(i, from.y()));
            }
        }
        Travel::Left(distance) => {
            for i in (from.x() - distance..from.x()).rev() {
                result.push(Point::new(i, from.y()));
            }
        }
    }

    result
}
