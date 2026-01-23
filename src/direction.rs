use macroquad::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight,
}

pub fn get_direction() -> Option<Direction> {
    let mut dir_vec = vec2(0.,0.);
    
    if is_key_down(KeyCode::Up) {
        dir_vec.y = -1.;
    } 
    if is_key_down(KeyCode::Down) {
        dir_vec.y = 1.; 
    }
     if is_key_down(KeyCode::Left) {
        dir_vec.x = -1.;
    }
    if is_key_down(KeyCode::Right) {
        dir_vec.x = 1.;
    }

    if dir_vec.x == 0. && dir_vec.y == -1. {
        Some(Direction::Up)
    } else if dir_vec.x == 0. && dir_vec.y == 1. {
        Some(Direction::Down)
    } else if dir_vec.x == -1. && dir_vec.y == 0. {
        Some(Direction::Left)
    } else if dir_vec.x == 1. && dir_vec.y == 0. {
        Some(Direction::Right)
    } else if dir_vec.x == 1. && dir_vec.y == 1. {
        Some(Direction::DownRight)
    } else if dir_vec.x == -1. && dir_vec.y == -1. {
        Some(Direction::UpLeft)
    } else if dir_vec.x == 1. && dir_vec.y == -1. {
        Some(Direction::UpRight)
    } else if dir_vec.x == -1. && dir_vec.y == 1. {
        Some(Direction::DownLeft)
    } else {
        None
    }
}
