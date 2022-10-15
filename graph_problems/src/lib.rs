use std::collections::{HashMap, VecDeque};

pub fn breath_first_search(graph: &HashMap<i32, Vec<i32>>, starting_node: i32) -> Vec<i32> {
    let mut queue: VecDeque<i32> = VecDeque::from([starting_node]);
    let mut result: Vec<i32> = vec![];
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        result.push(current_node);
        if let Some(childs) = graph.get(&current_node) {
            queue.append(&mut childs.to_owned().into());
        };
    }
    result
}

pub fn depth_first_search(graph: &HashMap<i32, Vec<i32>>, starting_node: i32) -> Vec<i32> {
    let mut stack: Vec<i32> = vec![starting_node];
    let mut result: Vec<i32> = vec![];
    while !stack.is_empty() {
        let current_node = stack.pop().unwrap();
        result.push(current_node);
        if let Some(childs) = graph.get(&current_node) {
            stack.append(&mut childs.to_owned());
        };
    }
    result
}

pub fn depth_first_search_rec(graph: &HashMap<i32, Vec<i32>>, starting_node: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    fn rec(result: &mut Vec<i32>, graph: &HashMap<i32, Vec<i32>>, node: i32) {
        result.push(node);
        if let Some(childs) = graph.get(&node) {
            for child in childs {
                rec(result, graph, *child)
            }
        }
    }
    rec(&mut result, graph, starting_node);
    result
}

pub fn create_graph_from_adjacency_list(edges: &[(i32, i32)]) -> HashMap<i32, Vec<i32>> {
    let mut graph = HashMap::<i32, Vec<i32>>::new();
    for (from, to) in edges {
        graph
            .entry(*from)
            .and_modify(|childs| childs.push(*to))
            .or_insert_with(|| vec![*to]);
        graph
            .entry(*to)
            .and_modify(|childs| childs.push(*from))
            .or_insert_with(|| vec![*from]);
    }
    graph
}

pub fn has_path_bfs(graph: &HashMap<i32, Vec<i32>>, starting_node: i32, destination: i32) -> bool {
    let mut queue = VecDeque::from([starting_node]);
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        if current_node == destination {
            return true;
        }
        if let Some(childs) = graph.get(&current_node) {
            queue.append(&mut childs.to_owned().into());
        }
    }
    false
}

pub fn has_path_dfs(graph: &HashMap<i32, Vec<i32>>, starting_node: i32, destination: i32) -> bool {
    let mut stack = vec![starting_node];
    while !stack.is_empty() {
        let current_node = stack.pop().unwrap();
        if current_node == destination {
            return true;
        }
        if let Some(childs) = graph.get(&current_node) {
            stack.append(&mut childs.to_owned());
        }
    }
    false
}

pub fn has_path_dfs_rec(
    graph: &HashMap<i32, Vec<i32>>,
    current_node: i32,
    destination: i32,
) -> bool {
    if current_node == destination {
        return true;
    }
    if let Some(childs) = graph.get(&current_node) {
        for child in childs {
            if has_path_dfs_rec(graph, *child, destination) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_breath_first_search() {
        let graph = HashMap::from([(1, vec![2, 3]), (2, vec![4]), (3, vec![5, 6])]);
        let result = breath_first_search(&graph, 1);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_depth_first_search() {
        let graph = HashMap::from([(1, vec![2, 3]), (2, vec![4]), (3, vec![5, 6])]);
        let result = depth_first_search(&graph, 1);
        assert_eq!(result, vec![1, 3, 6, 5, 2, 4]);
    }

    #[test]
    fn test_depth_first_search_rec() {
        let graph = HashMap::from([(1, vec![2, 3]), (2, vec![4]), (3, vec![5, 6])]);
        let result = depth_first_search_rec(&graph, 1);
        assert_eq!(result, vec![1, 2, 4, 3, 5, 6]);
    }

    #[test]
    fn test_create_graph_from_adjacency_list() {
        let edges = [(1, 2), (2, 3), (5, 3), (5, 4), (1, 4)];
        let result = create_graph_from_adjacency_list(&edges);
        assert_eq!(
            result,
            HashMap::from([
                (1, vec![2, 4]),
                (2, vec![1, 3]),
                (3, vec![2, 5]),
                (5, vec![3, 4]),
                (4, vec![5, 1])
            ])
        );
    }

    #[test]
    fn test_has_path() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![4]),
            (3, vec![5, 6]),
            (4, vec![]),
            (5, vec![]),
            (6, vec![]),
            (7, vec![8, 9, 10]),
            (8, vec![]),
            (9, vec![6]),
            (10, vec![]),
        ]);

        assert!(has_path_bfs(&graph, 7, 6));
    }

    #[test]
    fn test_has_path_dfs() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![4]),
            (3, vec![5, 6]),
            (4, vec![]),
            (5, vec![]),
            (6, vec![]),
            (7, vec![8, 9, 10]),
            (8, vec![]),
            (9, vec![6]),
            (10, vec![]),
        ]);

        assert!(has_path_dfs(&graph, 7, 6));
    }

    #[test]
    fn test_has_path_dfs_rec() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![4]),
            (3, vec![5, 6]),
            (4, vec![]),
            (5, vec![]),
            (6, vec![]),
            (7, vec![8, 9, 10]),
            (8, vec![]),
            (9, vec![6]),
            (10, vec![]),
        ]);

        assert!(has_path_dfs_rec(&graph, 7, 6));
    }
}
