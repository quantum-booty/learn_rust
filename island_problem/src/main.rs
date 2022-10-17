use std::collections::{HashSet, VecDeque};

fn main() {
    let matrix = vec![
        vec!['W', 'L', 'W', 'W', 'L', 'L'],
        vec!['W', 'W', 'W', 'W', 'W', 'W'],
        vec!['W', 'W', 'W', 'W', 'L', 'W'],
        vec!['W', 'L', 'L', 'W', 'L', 'L'],
        vec!['W', 'L', 'L', 'W', 'L', 'W'],
        vec!['W', 'W', 'W', 'W', 'W', 'W'],
    ];
    assert_eq!(count_islands_bfs(&matrix), 4);
    assert_eq!(count_islands_dfs_rec(&matrix), 4);
    assert_eq!(max_island_size_bfs(&matrix), 4);
    assert_eq!(max_island_size_dfs_rec(&matrix), 4);
}

fn count_islands_bfs(matrix: &Vec<Vec<char>>) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut count = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'W' {
                continue;
            }
            let coord = Coord::new(row, col);
            if visited.contains(&coord) {
                continue;
            }
            queue.push_back(coord);
            visited.insert(coord);
            while let Some(coord) = queue.pop_front() {
                for (coord, elem) in get_children(matrix, coord) {
                    if elem == 'W' || visited.contains(&coord) {
                        continue;
                    }
                    queue.push_back(coord);
                    visited.insert(coord);
                }
            }
            count += 1;
        }
    }
    count
}

fn count_islands_dfs_rec(matrix: &Vec<Vec<char>>) -> usize {
    fn is_island(
        matrix: &Vec<Vec<char>>,
        visited: &mut HashSet<Coord>,
        current_coord: Coord,
    ) -> bool {
        if matrix[current_coord.row][current_coord.col] == 'W' || visited.contains(&current_coord) {
            return false;
        }
        visited.insert(current_coord);
        for (coord, _) in get_children(matrix, current_coord) {
            is_island(matrix, visited, coord);
        }
        true
    }

    let mut visited = HashSet::new();
    let mut count = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if is_island(matrix, &mut visited, Coord::new(row, col)) {
                count += 1
            }
        }
    }
    count
}

fn max_island_size_bfs(matrix: &Vec<Vec<char>>) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut max_count = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'W' {
                continue;
            }
            let coord = Coord::new(row, col);
            if visited.contains(&coord) {
                continue;
            }
            queue.push_back(coord);
            visited.insert(coord);
            let mut count = 1;
            while let Some(coord) = queue.pop_front() {
                for (coord, elem) in get_children(matrix, coord) {
                    if elem == 'W' || visited.contains(&coord) {
                        continue;
                    }
                    count += 1;
                    queue.push_back(coord);
                    visited.insert(coord);
                }
            }
            max_count = max_count.max(count);
        }
    }
    max_count
}

fn max_island_size_dfs_rec(matrix: &Vec<Vec<char>>) -> usize {
    fn count_island(
        matrix: &Vec<Vec<char>>,
        visited: &mut HashSet<Coord>,
        current_coord: Coord,
    ) -> usize {
        if matrix[current_coord.row][current_coord.col] == 'W' || visited.contains(&current_coord) {
            return 0;
        }
        visited.insert(current_coord);
        let mut count = 1;
        for (coord, _) in get_children(matrix, current_coord) {
            count += count_island(matrix, visited, coord);
        }
        count
    }

    let mut visited = HashSet::new();
    let mut max_count = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let count = count_island(matrix, &mut visited, Coord::new(row, col));
            if count != 0 {
                max_count = max_count.max(count);
            }
        }
    }
    max_count
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn get_children<T>(matrix: &Vec<Vec<T>>, coord: Coord) -> Vec<(Coord, T)>
where
    T: Copy,
{
    let dim_rows = matrix.len();
    let dim_cols = matrix[0].len();
    let mut children = vec![];
    if coord.row > 0 {
        children.push((
            Coord::new(coord.row - 1, coord.col),
            matrix[coord.row - 1][coord.col],
        ));
    }
    if coord.row < dim_rows - 1 {
        children.push((
            Coord::new(coord.row + 1, coord.col),
            matrix[coord.row + 1][coord.col],
        ));
    }
    if coord.col > 0 {
        children.push((
            Coord::new(coord.row, coord.col - 1),
            matrix[coord.row][coord.col - 1],
        ));
    }
    if coord.col < dim_cols - 1 {
        children.push((
            Coord::new(coord.row, coord.col + 1),
            matrix[coord.row][coord.col + 1],
        ));
    }
    children
}
