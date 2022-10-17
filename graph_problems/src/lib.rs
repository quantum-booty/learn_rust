use std::collections::{HashMap, HashSet, VecDeque};

pub fn breath_first_search_directed(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
) -> Vec<i32> {
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

pub fn depth_first_search_directed(graph: &HashMap<i32, Vec<i32>>, starting_node: i32) -> Vec<i32> {
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

pub fn depth_first_search_rec_directed(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
) -> Vec<i32> {
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

pub fn has_path_bfs_directed(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
    destination: i32,
) -> bool {
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

pub fn has_path_dfs_directed(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
    destination: i32,
) -> bool {
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

pub fn has_path_dfs_rec_directed(
    graph: &HashMap<i32, Vec<i32>>,
    current_node: i32,
    destination: i32,
) -> bool {
    if current_node == destination {
        return true;
    }
    if let Some(childs) = graph.get(&current_node) {
        for child in childs {
            if has_path_dfs_rec_directed(graph, *child, destination) {
                return true;
            }
        }
    }
    false
}

pub fn has_path_bfs_undirected(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
    destination: i32,
) -> bool {
    let mut visited = HashSet::<i32>::new();
    let mut queue = VecDeque::from([starting_node]);
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();

        if visited.contains(&current_node) {
            continue;
        }
        visited.insert(current_node);

        if current_node == destination {
            return true;
        }
        if let Some(childs) = graph.get(&current_node) {
            queue.append(&mut childs.to_owned().into());
        }
    }
    false
}

pub fn has_path_dfs_undirected(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
    destination: i32,
) -> bool {
    let mut visited = HashSet::<i32>::new();
    let mut stack = vec![starting_node];
    while !stack.is_empty() {
        let current_node = stack.pop().unwrap();

        if visited.contains(&current_node) {
            continue;
        }
        visited.insert(current_node);

        if current_node == destination {
            return true;
        }
        if let Some(childs) = graph.get(&current_node) {
            stack.append(&mut childs.to_owned());
        }
    }
    false
}

pub fn has_path_dfs_rec_undirected(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
    destination: i32,
) -> bool {
    let mut visited = HashSet::<i32>::new();
    fn rec(
        graph: &HashMap<i32, Vec<i32>>,
        current_node: i32,
        destination: i32,
        visited: &mut HashSet<i32>,
    ) -> bool {
        if current_node == destination {
            return true;
        }

        if visited.contains(&current_node) {
            return false;
        }
        visited.insert(current_node);

        if let Some(childs) = graph.get(&current_node) {
            for child in childs {
                if rec(graph, *child, destination, visited) {
                    return true;
                }
            }
        }
        false
    }
    rec(graph, starting_node, destination, &mut visited)
}

pub fn connected_components_counts_bfs(graph: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut visited = HashSet::<i32>::new();

    for current_node in graph.keys() {
        if visited.contains(current_node) {
            continue;
        }
        let mut queue = VecDeque::<i32>::from(vec![*current_node]);
        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap();

            if visited.contains(&current_node) {
                continue;
            }
            visited.insert(current_node);

            if let Some(childs) = graph.get(&current_node) {
                queue.append(&mut childs.to_owned().into());
            }
        }
        count += 1;
    }
    count
}

pub fn connected_components_counts_dfs(graph: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut visited = HashSet::<i32>::new();

    for current_node in graph.keys() {
        if visited.contains(current_node) {
            continue;
        }
        let mut stack = vec![*current_node];
        while !stack.is_empty() {
            let current_node = stack.pop().unwrap();

            if visited.contains(&current_node) {
                continue;
            }
            visited.insert(current_node);

            if let Some(childs) = graph.get(&current_node) {
                stack.append(&mut childs.to_owned());
            }
        }
        count += 1;
    }
    count
}

pub fn connected_components_counts_rec_dfs(graph: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut visited = HashSet::<i32>::new();

    fn rec(graph: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>, current_node: i32) -> bool {
        if visited.contains(&current_node) {
            return false;
        }

        visited.insert(current_node);
        if let Some(childs) = graph.get(&current_node) {
            for child in childs {
                rec(graph, visited, *child);
            }
        }

        true
    }

    for current_node in graph.keys() {
        if rec(graph, &mut visited, *current_node) {
            count += 1;
        }
    }

    count
}

pub fn largest_component_bfs(graph: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut count: i32 = 0;
    let mut visited = HashSet::<i32>::new();

    for current_node in graph.keys() {
        if visited.contains(current_node) {
            continue;
        }
        let mut queue = VecDeque::<i32>::from(vec![*current_node]);
        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap();

            if visited.contains(&current_node) {
                continue;
            }
            visited.insert(current_node);

            if let Some(childs) = graph.get(&current_node) {
                queue.append(&mut childs.to_owned().into());
            }
        }
        count = count.max(visited.len().try_into().unwrap());
        visited.clear();
    }
    count
}

pub fn largest_component_dfs(graph: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut count: i32 = 0;
    let mut visited = HashSet::<i32>::new();

    for current_node in graph.keys() {
        if visited.contains(current_node) {
            continue;
        }
        let mut queue = vec![*current_node];
        while !queue.is_empty() {
            let current_node = queue.pop().unwrap();

            if visited.contains(&current_node) {
                continue;
            }
            visited.insert(current_node);

            if let Some(childs) = graph.get(&current_node) {
                queue.append(&mut childs.to_owned());
            }
        }
        count = count.max(visited.len().try_into().unwrap());
        visited.clear();
    }
    count
}

pub fn largest_component_rec_dfs(graph: &HashMap<i32, Vec<i32>>) -> i32 {
    fn rec(graph: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>, current_node: i32) {
        if !visited.contains(&current_node) {
            visited.insert(current_node);

            if let Some(childs) = graph.get(&current_node) {
                for child in childs {
                    rec(graph, visited, *child);
                }
            }
        }
    }

    let mut count: i32 = 0;
    let mut visited = HashSet::<i32>::new();

    for current_node in graph.keys() {
        if visited.contains(current_node) {
            continue;
        }

        rec(graph, &mut visited, *current_node);

        count = count.max(visited.len().try_into().unwrap());
        visited.clear();
    }
    count
}

pub fn shortest_path_bfs(
    graph: &HashMap<i32, Vec<i32>>,
    starting_node: i32,
    destination: i32,
) -> i32 {
    let mut visited = HashSet::<i32>::new();
    let mut queue = VecDeque::<(i32, i32)>::from([(starting_node, 0)]);

    while !queue.is_empty() {
        let (current_node, distance) = queue.pop_front().unwrap();

        if visited.contains(&current_node) {
            continue;
        }
        visited.insert(current_node);

        if current_node == destination {
            return distance;
        }

        if let Some(childs) = graph.get(&current_node) {
            for child in childs {
                queue.push_back((*child, distance + 1))
            }
        }
    }
    -1
}

pub fn cycle_in_graph(graph: &HashMap<i32, Vec<i32>>) -> bool {
    fn is_node_in_cycle(
        graph: &HashMap<i32, Vec<i32>>,
        node: i32,
        visited: &mut HashSet<i32>,
        currently_in_stack: &mut HashSet<i32>,
    ) -> bool {
        visited.insert(node);
        currently_in_stack.insert(node);

        if let Some(childs) = graph.get(&node) {
            for child in childs {
                if (!visited.contains(child)
                    && is_node_in_cycle(graph, *child, visited, currently_in_stack))
                    || currently_in_stack.contains(child)
                {
                    return true;
                }
            }
        }
        currently_in_stack.remove(&node);

        false
    }

    let mut visited = HashSet::<i32>::new();
    let mut currently_in_stack = HashSet::<i32>::new();

    for node in graph.keys() {
        if visited.contains(node) {
            continue;
        }
        if is_node_in_cycle(graph, *node, &mut visited, &mut currently_in_stack) {
            return true;
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
        let expected = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(breath_first_search_directed(&graph, 1), expected);
        let expected = vec![1, 2, 4, 3, 5, 6];
        assert_eq!(depth_first_search_rec_directed(&graph, 1), expected);
        let expected = vec![1, 3, 6, 5, 2, 4];
        assert_eq!(depth_first_search_directed(&graph, 1), expected);
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
    fn test_has_path_directed_graph() {
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

        assert!(has_path_bfs_directed(&graph, 7, 6));
        assert!(!has_path_bfs_directed(&graph, 7, 99));

        assert!(has_path_dfs_directed(&graph, 7, 6));
        assert!(!has_path_dfs_directed(&graph, 7, 99));

        assert!(has_path_dfs_rec_directed(&graph, 7, 6));
        assert!(!has_path_dfs_rec_directed(&graph, 7, 99));
    }

    #[test]
    fn test_has_path_undirected() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![1, 4]),
            (3, vec![1, 5, 6]),
            (4, vec![2]),
            (5, vec![3]),
            (6, vec![3]),
            (7, vec![8, 9, 10]),
            (8, vec![7]),
            (9, vec![7]),
            (10, vec![7]),
        ]);

        assert!(has_path_bfs_undirected(&graph, 2, 6));
        assert!(!has_path_bfs_undirected(&graph, 2, 99));

        assert!(has_path_dfs_undirected(&graph, 2, 6));
        assert!(!has_path_dfs_undirected(&graph, 2, 99));

        assert!(has_path_dfs_rec_undirected(&graph, 2, 6));
        assert!(!has_path_dfs_rec_undirected(&graph, 2, 99));
    }

    #[test]
    fn test_connected_components() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![1, 4]),
            (3, vec![1, 5, 6]),
            (4, vec![2]),
            (5, vec![3]),
            (6, vec![3]),
            (7, vec![8, 9, 10]),
            (8, vec![7]),
            (9, vec![7]),
            (10, vec![7]),
            (11, vec![12]),
            (12, vec![11]),
        ]);

        assert_eq!(connected_components_counts_bfs(&graph), 3);
        assert_eq!(connected_components_counts_dfs(&graph), 3);
        assert_eq!(connected_components_counts_rec_dfs(&graph), 3);
    }

    #[test]
    fn test_largest_component() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![1, 4]),
            (3, vec![1, 5, 6]),
            (4, vec![2]),
            (5, vec![3]),
            (6, vec![3]),
            (7, vec![8, 9, 10]),
            (8, vec![7]),
            (9, vec![7]),
            (10, vec![7]),
            (11, vec![12]),
            (12, vec![11]),
        ]);

        assert_eq!(largest_component_bfs(&graph), 6);
        assert_eq!(largest_component_dfs(&graph), 6);
        assert_eq!(largest_component_rec_dfs(&graph), 6);
    }

    #[test]
    fn test_shortest_path() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![1, 4]),
            (3, vec![1, 5, 6]),
            (4, vec![2, 5]),
            (5, vec![3, 4, 6]),
            (6, vec![3, 5]),
        ]);

        assert_eq!(shortest_path_bfs(&graph, 1, 6), 2);
    }

    #[test]
    fn test_cycle_in_graph() {
        let graph = HashMap::<i32, Vec<i32>>::from([
            (1, vec![2, 3]),
            (2, vec![3]),
            (3, vec![3]),
            (4, vec![5]),
            (5, vec![2]),
        ]);
        assert!(cycle_in_graph(&graph));

        let graph = HashMap::<i32, Vec<i32>>::from([(1, vec![2, 3]), (2, vec![4]), (3, vec![5])]);
        assert!(!cycle_in_graph(&graph));
    }
}
