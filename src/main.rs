use std::collections::HashSet;

#[derive(Debug)]
enum FileIOError{
    InvalidCharError(String),
    FileReadError(String)
}

fn main() -> Result<(), FileIOError> {
    let p1 = main_part_1()?;
    let p2 = main_part_2()?;
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    Ok(())
}

fn process_string(input: String, robo_santa: bool) -> Result<usize, FileIOError> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut santa_pos: (i32, i32) = (0, 0);
    let mut robo_santa_pos = (0,0);
    if robo_santa {
        robo_santa_pos = (0,0);
    }
    visited.insert(santa_pos);

    for (i,c) in input.chars().enumerate() {
        let pos: &mut (i32, i32);
        if robo_santa {
            pos = if i & 1 == 0 {&mut robo_santa_pos} else {&mut santa_pos};
        } else {
            pos = &mut santa_pos;
        }
        if c == '^' {
            pos.1 += 1; 
        }
        else if c == 'v' {
            pos.1 -= 1;
        }
        else if c == '<' {
            pos.0 -= 1;
        }
        else if c == '>' {
            pos.0 += 1;
        }
        //Ignore the end of file newline character.
        else if c == '\n' {
            continue;
        } 
        else {
            return Err(FileIOError::InvalidCharError(
                std::format!("Invalid Character {c}, expected one of: '^v<>\n'.").to_string()
            ));
        }
        visited.insert(*pos);    
    }
    Ok(visited.len())
}

fn main_part_1() -> Result<usize, FileIOError> {
    let input_file_name = "input.txt";
    let contents = match std::fs::read_to_string(input_file_name) {
        Ok(str) => str,
        Err(_) => {
            return Err(
                FileIOError::FileReadError(
                    std::format!("Could not read file {input_file_name}").to_string()
                ));
        }
    };
    let visited_count = process_string(contents, false)?; 
    Ok(visited_count)
}

fn main_part_2() -> Result<usize, FileIOError> {
    let input_file_name = "input.txt";
    let contents = match std::fs::read_to_string(input_file_name) {
        Ok(str) => str,
        Err(_) => {
            return Err(
                FileIOError::FileReadError(
                    std::format!("Could not read file {input_file_name}").to_string()
                ));
        }
    };
    let visited_count = process_string(contents, true)?;
    Ok(visited_count)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_single_moves() -> Result<(), FileIOError> {
        let test_pattern = "^>v<<".to_string();
        let expected_count_single_santa = 5;
        let expected_count_robo_santa = 4;
        assert_eq!(process_string(test_pattern.clone(), false)?, expected_count_single_santa);
        assert_eq!(process_string(test_pattern.clone(), true)?, expected_count_robo_santa);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<(), FileIOError> {
        assert_eq!(main_part_1()?, 2565);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), FileIOError> {
        assert_eq!(main_part_2()?, 2639);
        Ok(())
    }

    // #[test]
    // fn test_visited_houses_new() {
    //     let mut visited_houses = VisitedHouses::new();
    //     assert_eq!(visited_houses.num_visited_houses(), 1);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_direction_try_from() {
    //     assert_eq!('^'.try_into(), Ok(Direction::North));
    //     assert_eq!('v'.try_into(), Ok(Direction::South));
    //     assert_eq!('<'.try_into(), Ok(Direction::West));
    //     assert_eq!('>'.try_into(), Ok(Direction::East));
    //     assert_eq!('x'.try_into(), Err(IllegalDirectionCharacter('x')));
    // }

    // #[test]
    // fn test_move_east() {
    //     let mut visited_houses = VisitedHouses::new();
    //     visited_houses.perform_move(Direction::East);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(1, 0));
    // }

    // #[test]
    // fn test_square_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^>v<").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 4);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_up_down_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^v^v^v^v^v").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_aoc_input() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str(include_str!("../input.txt")).unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2565);
    // }
}
