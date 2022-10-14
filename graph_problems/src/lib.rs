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
}
