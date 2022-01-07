use std::collections::VecDeque;

use crate::direction::Direction;
use crate::point::Point;
use crate::resources::{snake_colors, take_random_colors};
use crate::snake::Snake;

pub fn map1() -> Vec<(i32, i32)> {
    return vec![
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9),
        (0, 10), (0, 11), (0, 12), (0, 13), (0, 14), (0, 15), (0, 16), (0, 17), (0, 18), (0, 19),
        (0, 20), (0, 21), (0, 22), (0, 23), (0, 24), (0, 25), (0, 26), (0, 27), (0, 28), (0, 29),

        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0),
        (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0),
        (21, 0), (22, 0), (23, 0), (24, 0), (25, 0), (26, 0), (27, 0), (28, 0), (29, 0),

        (1, 29), (2, 29), (3, 29), (4, 29), (5, 29), (6, 29), (7, 29), (8, 29), (9, 29), (10, 29),
        (11, 29), (12, 29), (13, 29), (14, 29), (15, 29), (16, 29), (17, 29), (18, 29), (19, 29), (20, 29),
        (21, 29), (22, 29), (23, 29), (24, 29), (25, 29), (26, 29), (27, 29), (28, 29), (29, 29),

        (29, 1), (29, 2), (29, 3), (29, 4), (29, 5), (29, 6), (29, 7), (29, 8), (29, 9),
        (29, 10), (29, 11), (29, 12), (29, 13), (29, 14), (29, 15), (29, 16), (29, 17), (29, 18), (29, 19),
        (29, 20), (29, 21), (29, 22), (29, 23), (29, 24), (29, 25), (29, 26), (29, 27), (29, 28),
    ];
}

pub fn map2() -> Vec<(i32, i32)> {
    return vec![];
}

pub fn map3() -> Vec<(i32, i32)> {
    return vec![
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9),
        (0, 10), (0, 11), (0, 12), (0, 17), (0, 18), (0, 19),
        (0, 20), (0, 21), (0, 22), (0, 23), (0, 24), (0, 25), (0, 26), (0, 27), (0, 28), (0, 29),

        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0),
        (11, 0), (12, 0), (17, 0), (18, 0), (19, 0), (20, 0),
        (21, 0), (22, 0), (23, 0), (24, 0), (25, 0), (26, 0), (27, 0), (28, 0), (29, 0),

        (1, 29), (2, 29), (3, 29), (4, 29), (5, 29), (6, 29), (7, 29), (8, 29), (9, 29), (10, 29),
        (11, 29), (12, 29), (17, 29), (18, 29), (19, 29), (20, 29),
        (21, 29), (22, 29), (23, 29), (24, 29), (25, 29), (26, 29), (27, 29), (28, 29), (29, 29),

        (29, 1), (29, 2), (29, 3), (29, 4), (29, 5), (29, 6), (29, 7), (29, 8), (29, 9),
        (29, 10), (29, 11), (29, 12), (29, 17), (29, 18), (29, 19),
        (29, 20), (29, 21), (29, 22), (29, 23), (29, 24), (29, 25), (29, 26), (29, 27), (29, 28),

        (13, 13), (13, 14), (13, 15), (13, 16),
        (14, 13), (14, 14), (14, 15), (14, 16),
        (15, 13), (15, 14), (15, 15), (15, 16),
        (16, 13), (16, 14), (16, 15), (16, 16)
    ];
}

pub fn map4() -> Vec<(i32, i32)> {
    return vec![
        (0, 13), (1, 13), (2, 13), (3, 13), (4, 13), (5, 13), (6, 13), (7, 13), (8, 13), (9, 13), (10, 13), (11, 13), (12, 13), (13, 13),
        (13, 0), (13, 1), (13, 2), (13, 3), (13, 4), (13, 5), (13, 6), (13, 7), (13, 8), (13, 9), (13, 10), (13, 11), (13, 12),

        (17, 17), (17, 18), (17, 19), (17, 20), (17, 21), (17, 22), (17, 23), (17, 24), (17, 25), (17, 26), (17, 27), (17, 28), (17, 29),
        (18, 17), (19, 17), (20, 17), (21, 17), (22, 17), (23, 17), (24, 17), (25, 17), (26, 17), (27, 17), (28, 17), (29, 17)
    ];
}

pub fn map5() -> Vec<(i32, i32)> {
    return vec![
        (1, 1), (2, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (11, 1), (12, 1),
        (18, 1), (19, 1), (20, 1), (21, 1), (22, 1), (23, 1), (24, 1), (25, 1), (26, 1), (27, 1), (28, 1),

        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8),

        (1, 28), (2, 28), (2, 28), (3, 28), (4, 28), (5, 28), (6, 28), (7, 28), (8, 28), (9, 28), (10, 28), (11, 28), (12, 28),
        (18, 28), (19, 28), (20, 28), (21, 28), (22, 28), (23, 28), (24, 28), (25, 28), (26, 28), (27, 28), (28, 28),

        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (1, 10), (1, 11),
        (1, 27), (1, 26), (1, 25), (1, 24), (1, 23), (1, 22), (1, 21), (1, 20), (1, 19), (1, 18),

        (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11),
        (2, 18), (3, 18), (4, 18), (5, 18), (6, 18), (7, 18),

        (7, 12), (7, 13), (7, 14), (7, 15), (7, 16), (7, 17),

        (12, 2), (12, 3), (12, 4), (12, 5), (12, 6),
        (18, 2), (18, 3), (18, 4), (18, 5), (18, 6),

        (13, 6), (14, 6), (15, 6), (16, 6), (17, 6),

        (12, 27), (12, 26), (12, 25), (12, 24), (12, 23),
        (18, 27), (18, 26), (18, 25), (18, 24), (18, 23),

        (28, 18), (28, 19), (28, 20), (28, 21), (28, 22), (28, 23), (28, 24), (28, 25), (28, 26), (28, 27), (28, 28),
        (23, 18), (24, 18), (25, 18), (26, 18), (27, 18),

        (28, 2), (28, 3), (28, 4), (28, 5), (28, 6), (28, 7), (28, 8), (28, 9), (28, 10), (28, 11),
        (23, 11), (24, 11), (25, 11), (26, 11), (27, 11), (28, 11),
    ];
}

pub fn map_and_snake1() -> (Vec<(i32, i32)>, Snake) {
    let walls = map1();
    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((15, 15));
    deque.push_back((15, 16));
    deque.push_back((15, 17));

    let (snake_body_color, snake_stroke_color) = take_random_colors(snake_colors());
    let snake = Snake::new(15., 15., Direction::TOP, deque, snake_body_color, snake_stroke_color);

    return (walls, snake);
}

pub fn map_and_snake2() -> (Vec<(i32, i32)>, Snake) {
    let walls = map2();
    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((15, 15));
    deque.push_back((15, 16));
    deque.push_back((15, 17));

    let (snake_body_color, snake_stroke_color) = take_random_colors(snake_colors());
    let snake = Snake::new(15., 15., Direction::TOP, deque, snake_body_color, snake_stroke_color);

    return (walls, snake);
}

pub fn map_and_snake3() -> (Vec<(i32, i32)>, Snake) {
    let walls = map3();
    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((2, 15));
    deque.push_back((1, 15));
    deque.push_back((0, 15));

    let (snake_body_color, snake_stroke_color) = take_random_colors(snake_colors());
    let snake = Snake::new(2., 15., Direction::RIGHT, deque, snake_body_color, snake_stroke_color);

    return (walls, snake);
}

pub fn map_and_snake4() -> (Vec<(i32, i32)>, Snake) {
    let walls = map4();
    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((2, 15));
    deque.push_back((1, 15));
    deque.push_back((0, 15));

    let (snake_body_color, snake_stroke_color) = take_random_colors(snake_colors());
    let snake = Snake::new(2., 15., Direction::RIGHT, deque, snake_body_color, snake_stroke_color);

    return (walls, snake);
}

pub fn map_and_snake5() -> (Vec<(i32, i32)>, Snake) {
    let walls = map5();
    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((2, 15));
    deque.push_back((1, 15));
    deque.push_back((0, 15));

    let (snake_body_color, snake_stroke_color) = take_random_colors(snake_colors());
    let snake = Snake::new(2., 15., Direction::RIGHT, deque, snake_body_color, snake_stroke_color);

    return (walls, snake);
}
