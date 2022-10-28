use hashbrown::HashSet;
use std::collections::VecDeque;

fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), 2);
    let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), 4);
    let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    y: usize,
    x: usize,
}

fn get_neighbours(grid: &Vec<Vec<i32>>, coord: Coord) -> Vec<Coord> {
    let mut neighbours = Vec::new();
    let y = coord.y;
    let x = coord.x;

    if y > 0 {
        if x > 0 {
            neighbours.push(Coord { y: y - 1, x: x - 1 });
        }
        neighbours.push(Coord { y: y - 1, x });
        if x < grid[0].len() - 1 {
            neighbours.push(Coord { y: y - 1, x: x + 1 });
        }
    }
    if x > 0 {
        neighbours.push(Coord { y, x: x - 1 });
    }
    if x < grid[0].len() - 1 {
        neighbours.push(Coord { y, x: x + 1 });
    }
    if y < grid.len() - 1 {
        if x > 0 {
            neighbours.push(Coord { y: y + 1, x: x - 1 });
        }
        neighbours.push(Coord { y: y + 1, x });
        if x < grid[0].len() - 1 {
            neighbours.push(Coord { y: y + 1, x: x + 1 });
        }
    }
    neighbours
}

fn breadth_first_search(grid: Vec<Vec<i32>>, start: Coord, destination: Coord) -> i32 {
    let mut queue = VecDeque::<(Coord, i32)>::new();
    let mut visited = HashSet::<Coord>::new();
    queue.push_front((start, 1));

    while let Some((current_coord, cost)) = queue.pop_back() {
        if current_coord == destination {
            return cost;
        }
        if visited.contains(&current_coord) {
            continue;
        }
        visited.insert(current_coord);

        for child in get_neighbours(&grid, current_coord) {
            if grid[child.y][child.x] == 0 {
                queue.push_front((child, cost + 1));
            }
        }
    }

    -1
}

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let start = Coord { y: 0, x: 0 };
        if grid[start.y][start.y] != 0 {
            return -1;
        }
        let destination = Coord {
            y: grid.len() - 1,
            x: grid[0].len() - 1,
        };
        breadth_first_search(grid, start, destination)
    }
}
