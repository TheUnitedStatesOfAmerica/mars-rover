use mars_rover::rover;
use mars_rover::rover::*;

#[test]
fn test_parse_world() {
    let input = "5 5";
    let output = rover::parse_world(input).expect("could not parse world");
    assert!(output == (World { x: 5, y: 5 }));
}

#[test]
fn test_parse_position() {
    let input = "1 5 E";
    let output: (Position, Direction) =
        rover::parse_position(input).expect("could not parse location and direction");
    assert!(output == (Position { x: 1, y: 5 }, Direction::East));
}

#[test]
fn test_rover_init() {
    let input: &str = "5 5\n1 2 N\nLMLMLMLMM\n3 3 E\nMMRMMRMRRM";
    let mut lines = input.lines();

    let world = rover::parse_world(lines.next().unwrap()).expect("could not parse world");
    let rover1_initial_data: (Position, Direction) = rover::parse_position(lines.next().unwrap())
        .expect("could not parse location and direction");

    let mut rover1 = Rover::new(rover1_initial_data.0, rover1_initial_data.1, world);
    rover1
        .run_to_end(lines.next().unwrap())
        .expect("could not complete moves");

    let rover2_initial_data: (Position, Direction) = rover::parse_position(lines.next().unwrap())
        .expect("could not parse location and direction");
    let mut rover2 = Rover::new(rover2_initial_data.0, rover2_initial_data.1, world);
    rover2
        .run_to_end(lines.next().unwrap())
        .expect("could not complete moves");

    println!("{:?}", rover1.position);
    println!("{:?}", rover1.direction);
    println!("{:?}", rover2.position);
    println!("{:?}", rover2.direction);

    assert!(rover1.position == (Position { x: 1, y: 3 }));
    assert!(rover1.world == (World { x: 5, y: 5 }));
    assert!(rover1.direction == Direction::North);
    assert!(rover2.position == (Position { x: 5, y: 1 }));
    assert!(rover2.world == (World { x: 5, y: 5 }));
    assert!(rover2.direction == Direction::East);
}
