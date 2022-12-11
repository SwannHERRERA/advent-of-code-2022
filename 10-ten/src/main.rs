mod timer;
use std::fs;
use timer::*;

fn parse_line(line: &str) -> Command {
    if line == "noop" {
        return Command::Noop;
    }
    let (_, value) = line.split_once(' ').unwrap();
    let value: i64 = value.parse().unwrap();
    Command::AddX(value)
}
fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(parse_line).collect()
}

fn main() {
    let input = fs::read_to_string("10-ten/input.txt").unwrap();
    let commands = parse_input(&input);
    let mut timer = Timer::new();
    for command in commands {
        timer.next_cycle(command);
    }
    let part_one = timer.get_sum();
    println!("{:?}", timer);
    println!("part one : {}", part_one);
    let part_two = timer.get_ctr();
    println!("{}", part_two);
}
