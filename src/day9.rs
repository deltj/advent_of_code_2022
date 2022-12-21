use std::{io::BufRead, collections::HashSet};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq)]
pub struct Movement {
    pub direction: Direction,
    pub distance: i32,
}

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

fn move_head(pos: (i32, i32), movement: &Movement) -> (i32, i32) {
    match movement.direction {
        Direction::Up    => (pos.0, pos.1 + movement.distance),
        Direction::Down  => (pos.0, pos.1 - movement.distance),
        Direction::Left  => (pos.0 - movement.distance, pos.1),
        Direction::Right => (pos.0 + movement.distance, pos.1),
    }
}

fn move_tail(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let mut new_tail_pos = tail_pos;

    let dx = (head_pos.0 - tail_pos.0).abs();

    if dx > 1 {
        if head_pos.0 > tail_pos.0 {
            //  Move right
            new_tail_pos.0 += dx - 1;
        } else {
            //  Move left
            new_tail_pos.0 -= dx - 1;
        }
    }

    if dx >= 2 {
        //  Align head and tail on y axis
        new_tail_pos.1 = head_pos.1;
    }

    let dy = (head_pos.1 - tail_pos.1).abs();

    if dy > 1 {
        if head_pos.1 > tail_pos.1 {
            //  Move up
            new_tail_pos.1 += dy - 1;
        } else {
            //  Move down
            new_tail_pos.1 -= dy - 1;
        }
    }

    if dy >= 2 {
        //  Align head and tail on x axis
        new_tail_pos.0 = head_pos.0;
    }

    return new_tail_pos;
}

pub fn process_movements(head_pos: &mut (i32, i32), tail_pos: &mut (i32, i32), movements: &Vec<Movement>, history: &mut HashSet<(i32, i32)>) {
    for movement in movements {
        //  break each movement into smaller movements of distance 1
        let lil_movement = Movement { direction: movement.direction, distance: 1};
        for _ in 0..movement.distance {

            //  Move the rope head
            *head_pos = move_head(*head_pos, &lil_movement);
            //println!("new head pos is ({0}, {1})", head_pos.0, head_pos.1);

            //  Move the rope tail
            *tail_pos = move_tail(*head_pos, *tail_pos);
            //println!("new tail pos is ({0}, {1})", tail_pos.0, tail_pos.1);

            //  Update history
            history.insert(*tail_pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_rope_up() {
        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        let movement = Movement { direction: Direction::Up, distance: 4};
        head_pos = move_head(head_pos, &movement);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((0, 4), head_pos);
        assert_eq!((0, 3), tail_pos);
    }

    #[test]
    fn move_rope_down() {
        let mut head_pos = (0, 4);
        let mut tail_pos = (0, 4);
        let movement = Movement { direction: Direction::Down, distance: 4};
        head_pos = move_head(head_pos, &movement);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((0, 0), head_pos);
        assert_eq!((0, 1), tail_pos);
    }

    #[test]
    fn move_rope_left() {
        let mut head_pos = (4, 0);
        let mut tail_pos = (4, 0);
        let movement = Movement { direction: Direction::Left, distance: 4};
        head_pos = move_head(head_pos, &movement);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((0, 0), head_pos);
        assert_eq!((1, 0), tail_pos);
    }

    #[test]
    fn move_rope_right() {
        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        let movement = Movement { direction: Direction::Right, distance: 4};
        head_pos = move_head(head_pos, &movement);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((4, 0), head_pos);
        assert_eq!((3, 0), tail_pos);
    }

    #[test]
    fn ayy_lmao() {
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

        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        
        head_pos = move_head(head_pos, &movements[0]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((4, 0), head_pos);
        assert_eq!((3, 0), tail_pos);

        head_pos = move_head(head_pos, &movements[1]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((4, 4), head_pos);
        assert_eq!((4, 3), tail_pos);

        head_pos = move_head(head_pos, &movements[2]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((1, 4), head_pos);
        assert_eq!((2, 4), tail_pos);

        head_pos = move_head(head_pos, &movements[3]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((1, 3), head_pos);
        assert_eq!((2, 4), tail_pos);

        head_pos = move_head(head_pos, &movements[4]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((5, 3), head_pos);
        assert_eq!((4, 3), tail_pos);

        head_pos = move_head(head_pos, &movements[5]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((5, 2), head_pos);
        assert_eq!((4, 3), tail_pos);

        head_pos = move_head(head_pos, &movements[6]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((0, 2), head_pos);
        assert_eq!((1, 2), tail_pos);

        head_pos = move_head(head_pos, &movements[7]);
        tail_pos = move_tail(head_pos, tail_pos);

        assert_eq!((2, 2), head_pos);
        assert_eq!((1, 2), tail_pos);
    }

    #[test]
    fn ayy_lmao2() {
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

        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        let mut history: HashSet<(i32, i32)> = HashSet::new();

        process_movements(&mut head_pos, &mut tail_pos, &movements, &mut history);
        assert_eq!((2, 2), head_pos);
        assert_eq!((1, 2), tail_pos);

        let visited_positions = history.len();
        assert_eq!(13, visited_positions);
    }
}
