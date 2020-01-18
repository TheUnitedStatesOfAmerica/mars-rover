use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct World {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn forward(&mut self, direction: Direction, world: World) {
        let diff = match direction {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        if self.x + diff.0 >= 0 && self.x + diff.0 <= world.x {
            self.x += diff.0;
        }
        if self.y + diff.1 >= 0 && self.y + diff.1 <= world.y {
            self.y += diff.1;
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&mut self) {
        *self = match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(&mut self) {
        *self = match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
struct InvalidCommand;
impl Error for InvalidCommand {}
impl fmt::Display for InvalidCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid command")
    }
}

pub struct Rover {
    pub position: Position,
    pub direction: Direction,
    pub world: World,
}

enum Command {
    Left,
    Right,
    Forward,
}

impl Rover {
    pub fn new(position: Position, direction: Direction, world: World) -> Self {
        Self::_new(position, direction, world)
    }

    fn _new(position: Position, direction: Direction, world: World) -> Self {
        Self {
            position,
            direction,
            world,
        }
    }

    /// Takes the second line of input following the 'LMR' format
    /// Invalid input not accounted for, that should be filtered by the implementation of what runs the rover, this is just meant to parse
    pub fn run_to_end(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let commands = input.chars().map(map_to_command).collect::<Vec<_>>();
        for command in commands {
            match command? {
                Command::Left => self.direction.left(),
                Command::Right => self.direction.right(),
                Command::Forward => self.position.forward(self.direction, self.world),
            }
        }

        Ok(())
    }
}

/// Parses a line of input following the `int int` format
/// Invalid input not accounted for, that should be filtered by the implementation of what runs the rover, this is just meant to parse
pub fn parse_world(line: &str) -> Result<World, Box<dyn Error>> {
    let coords = line.split_whitespace().collect::<Vec<&str>>();
    Ok(World {
        x: coords[0].parse()?,
        y: coords[1].parse()?,
    })
}

/// Parses a line of input following the `int int str` format
/// Invalid input not accounted for, that should be filtered by the implementation of what runs the rover, this is just meant to parse
pub fn parse_position(line: &str) -> Result<(Position, Direction), Box<dyn Error>> {
    let mut char = line.split_whitespace();
    let x: i32 = char.next().unwrap().parse()?;
    let y: i32 = char.next().unwrap().parse()?;
    let d = char.next().unwrap();

    let direction = match d {
        "N" => Direction::North,
        "E" => Direction::East,
        "S" => Direction::South,
        "W" => Direction::West,
        _ => Direction::North,
    };

    Ok((Position { x, y }, direction))
}

/// Maps characters from `str` to a `Command`
/// Returns a `Command` if valid, a boxed Error that must be handled at some point if not
fn map_to_command(c: char) -> Result<Command, Box<dyn Error>> {
    match c {
        'M' => Ok(Command::Forward),
        'R' => Ok(Command::Right),
        'L' => Ok(Command::Left),
        _ => Err(Box::new(InvalidCommand)),
    }
}
