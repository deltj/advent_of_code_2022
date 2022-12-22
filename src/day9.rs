use std::{io::BufRead, collections::HashSet};

/// Enum to describe movement direction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

/// Structure to describe a movement instruction from input
#[derive(Debug, PartialEq)]
pub struct Movement {
    pub direction: Direction,
    pub distance: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Structure to store the x,y coordinates of a rope vertex (knot)
pub struct RopeVertex {
    pub x: i32,
    pub y: i32,
}

/// A rope is a collection of rope vertices
pub struct Rope {
    pub vertices: Vec<RopeVertex>,
}

impl Rope {
    /// Create a new Rope with the specified number of verticies.
    /// (vertex 0 is the head of the rope, and vertex n-1 is the tail)
    pub fn new(num_vertices: u32) -> Self {
        let mut vertices: Vec<RopeVertex> = Vec::new();

        for _ in 0..num_vertices {
            vertices.push(RopeVertex { x: 0, y: 0 });
        }

        Self { vertices }
    }
}

/// Read input movements into a vector
pub fn read_movements(reader: &mut dyn BufRead) -> Vec<Movement> {
    let mut rmv: Vec<Movement> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();

        let mut rm = Movement{ direction : Direction::Up, distance : 0, };

        let tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
        if tokens.len() != 2 {
            panic!("Unexpected number of tokens");
        }

        match tokens[0] {
            "U" => rm.direction = Direction::Up,
            "D" => rm.direction = Direction::Down,
            "L" => rm.direction = Direction::Left,
            "R" => rm.direction = Direction::Right,
            _ => panic!("invalid direction"),
        }

        rm.distance = tokens[1].parse::<i32>().unwrap();

        rmv.push(rm);
    }

    return rmv;
}

/// Move the head of the rope one unit in the specified direction
fn move_head(pos: &mut RopeVertex, direction: Direction) {
    match direction {
        Direction::Up    => pos.y += 1,
        Direction::Down  => pos.y -= 1,
        Direction::Left  => pos.x -= 1,
        Direction::Right => pos.x += 1,
    }
}

/// Move the vertex at the specified index (if necessary)
fn move_vertex(rope: &mut Rope, index: usize) {

    let lead_vertex = rope.vertices[index - 1];
    let mut vertex = &mut rope.vertices[index];

    let dx = (lead_vertex.x - vertex.x).abs();
    let dy = (lead_vertex.y - vertex.y).abs();

    let mut move_x = 1;
    let mut move_y = 1;

    if vertex.x > lead_vertex.x {
        move_x = -1;
    }

    if vertex.y > lead_vertex.y {
        move_y = -1;
    }

    if dx > 1 && dy == 0 {
        //  Vertex needs to move along x axis only
        vertex.x += move_x;
    } else if dy > 1 && dx == 0 {
        //  Vertex needs to move along y axis only
        vertex.y += move_y;
    } else if (dx > 1 && dy > 0) || (dx > 0 && dy > 1) {
        //  Vertex needs to move along x and y axes
        vertex.x += move_x;
        vertex.y += move_y;
    }
}

/// Move the rope according to the specified Movement
fn move_rope(rope: &mut Rope, movement: &Movement) {
    //  Move the head according to the specified movement
    move_head(&mut rope.vertices[0], movement.direction);

    //  Move the other vertices in the rope
    for i in 1..rope.vertices.len() {
        move_vertex(rope, i);
    }
}

/// Apply a set of movements to the specified rope
pub fn process_movements(rope: &mut Rope, movements: &Vec<Movement>, history: &mut HashSet<RopeVertex>) {
    for movement in movements {
        //  break each movement into smaller movements of distance 1
        let lil_movement = Movement { direction: movement.direction, distance: 1};
        for _ in 0..movement.distance {

            //  Move the rope
            move_rope(rope, &lil_movement);

            let tail_vertex = rope.vertices[rope.vertices.len() - 1];

            //  Update history
            history.insert(tail_vertex);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_rope_up() {
        let mut rope: Rope = Rope::new(5);
        let mut movements: Vec<Movement> = Vec::new();
        movements.push(Movement { direction: Direction::Up, distance: 4});

        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);

        assert_eq!(RopeVertex { x: 0, y: 4 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: 0, y: 3 }, rope.vertices[1]);
        assert_eq!(RopeVertex { x: 0, y: 2 }, rope.vertices[2]);
        assert_eq!(RopeVertex { x: 0, y: 1 }, rope.vertices[3]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[4]);
    }

    #[test]
    fn move_rope_down() {
        let mut rope: Rope = Rope::new(5);
        let mut movements: Vec<Movement> = Vec::new();
        movements.push(Movement { direction: Direction::Down, distance: 4});

        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);

        assert_eq!(RopeVertex { x: 0, y: -4 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: 0, y: -3 }, rope.vertices[1]);
        assert_eq!(RopeVertex { x: 0, y: -2 }, rope.vertices[2]);
        assert_eq!(RopeVertex { x: 0, y: -1 }, rope.vertices[3]);
        assert_eq!(RopeVertex { x: 0, y:  0 }, rope.vertices[4]);
    }

    #[test]
    fn move_rope_left() {
        let mut rope: Rope = Rope::new(5);
        let mut movements: Vec<Movement> = Vec::new();
        movements.push(Movement { direction: Direction::Left, distance: 4});

        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);

        assert_eq!(RopeVertex { x: -4, y: 0 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: -3, y: 0 }, rope.vertices[1]);
        assert_eq!(RopeVertex { x: -2, y: 0 }, rope.vertices[2]);
        assert_eq!(RopeVertex { x: -1, y: 0 }, rope.vertices[3]);
        assert_eq!(RopeVertex { x:  0, y: 0 }, rope.vertices[4]);
    }

    #[test]
    fn move_rope_right() {
        let mut rope: Rope = Rope::new(5);
        let mut movements: Vec<Movement> = Vec::new();
        movements.push(Movement { direction: Direction::Right, distance: 4});

        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);

        assert_eq!(RopeVertex { x: 4, y: 0 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: 3, y: 0 }, rope.vertices[1]);
        assert_eq!(RopeVertex { x: 2, y: 0 }, rope.vertices[2]);
        assert_eq!(RopeVertex { x: 1, y: 0 }, rope.vertices[3]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[4]);
    }

    #[test]
    fn r5() {
        let input = "R 5";
        let mut buf = input.as_bytes();
        let movements = read_movements(&mut buf);
        assert_eq!(1, movements.len());

        let mut rope: Rope = Rope::new(10);
        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);

        assert_eq!(RopeVertex { x: 5, y: 0 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: 4, y: 0 }, rope.vertices[1]);
        assert_eq!(RopeVertex { x: 3, y: 0 }, rope.vertices[2]);
        assert_eq!(RopeVertex { x: 2, y: 0 }, rope.vertices[3]);
        assert_eq!(RopeVertex { x: 1, y: 0 }, rope.vertices[4]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[5]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[6]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[7]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[8]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[9]);
    }

    #[test]
    fn r5u8() {
        let input = "R 5
            U 8";
        let mut buf = input.as_bytes();
        let movements = read_movements(&mut buf);
        assert_eq!(2, movements.len());

        let mut rope: Rope = Rope::new(10);
        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);

        assert_eq!(RopeVertex { x: 5, y: 8 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: 5, y: 7 }, rope.vertices[1]);
        assert_eq!(RopeVertex { x: 5, y: 6 }, rope.vertices[2]);
        assert_eq!(RopeVertex { x: 5, y: 5 }, rope.vertices[3]);
        assert_eq!(RopeVertex { x: 5, y: 4 }, rope.vertices[4]);
        assert_eq!(RopeVertex { x: 4, y: 4 }, rope.vertices[5]);
        assert_eq!(RopeVertex { x: 3, y: 3 }, rope.vertices[6]);
        assert_eq!(RopeVertex { x: 2, y: 2 }, rope.vertices[7]);
        assert_eq!(RopeVertex { x: 1, y: 1 }, rope.vertices[8]);
        assert_eq!(RopeVertex { x: 0, y: 0 }, rope.vertices[9]);
    }

    #[test]
    fn ayy_lmao1() {
        let input = "R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2";
        let mut buf = input.as_bytes();
        let movements = read_movements(&mut buf);
        assert_eq!(8, movements.len());

        assert_eq!(Movement { direction: Direction::Right, distance: 4}, movements[0]);
        assert_eq!(Movement { direction: Direction::Up, distance: 4}, movements[1]);
        assert_eq!(Movement { direction: Direction::Left, distance: 3}, movements[2]);
        assert_eq!(Movement { direction: Direction::Down, distance: 1}, movements[3]);

        //  Part 1 test
        let mut rope: Rope = Rope::new(2);
        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);
        assert_eq!(RopeVertex { x: 2, y: 2 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: 1, y: 2 }, rope.vertices[1]);

        let visited_positions = history.len();
        assert_eq!(13, visited_positions);

        //  Part 2 test
        let mut rope: Rope = Rope::new(10);
        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);
        assert_eq!(RopeVertex { x: 2, y: 2 }, rope.vertices[0]);
        assert_eq!(RopeVertex { x: 1, y: 2 }, rope.vertices[1]);

        let visited_positions = history.len();
        assert_eq!(1, visited_positions);
    }

    #[test]
    fn ayy_lmao2() {
        let input = "R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20";
        let mut buf = input.as_bytes();
        let movements = read_movements(&mut buf);
        assert_eq!(8, movements.len());

        let mut rope: Rope = Rope::new(10);

        let mut history: HashSet<RopeVertex> = HashSet::new();
        process_movements(&mut rope, &movements, &mut history);

        let visited_positions = history.len();
        assert_eq!(36, visited_positions);
    }
}
