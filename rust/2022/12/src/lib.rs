use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum NodeType {
    Start,
    End,
    Intermediate(u8),
}

impl NodeType {
    fn height(self) -> u8 {
        match self {
            NodeType::Start => 25,
            NodeType::End => 0,
            NodeType::Intermediate(h) => h,
        }
    }

    fn is_starting(self) -> bool {
        match self {
            NodeType::Start => true,
            _ => false,
        }
    }

    fn is_finishing(self) -> bool {
        match self {
            NodeType::End => true,
            _ => false,
        }
    }
}

impl From<char> for NodeType {
    fn from(ch: char) -> NodeType {
        match ch {
            'E' => NodeType::Start,
            'S' => NodeType::End,
            'a' => NodeType::End,
            'b'..='z' => NodeType::Intermediate(ch as u8 - b'a'),
            _ => panic!("invalid character: {ch}"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    coordinate: (usize, usize),
    node_type: NodeType,
}

pub struct Graph {
    width: usize,
    height: usize,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut x = 0;
        let mut y = 0;
        let mut nodes = vec![];

        for (_, ch) in input.chars().enumerate() {
            let node_type = match ch {
                '\r' | '\n' => continue,
                _ => NodeType::from(ch),
            };

            nodes.push(Node {
                coordinate: (x, y),
                node_type,
            });

            if x == width - 1 {
                x = 0;
                y += 1;
            } else {
                x += 1;
            }
        }

        Self {
            width,
            height,
            nodes,
        }
    }

    pub fn bfs(&self) -> Option<i32> {
        let current = self
            .nodes
            .iter()
            .find(|x| x.node_type.is_starting())
            .unwrap();

        let mut current_nodes = vec![current.clone()];
        let mut count = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while !current_nodes.is_empty() {
            let mut next_round: Vec<Node> = vec![];

            for _ in 0..current_nodes.len() {
                let node = current_nodes.pop().unwrap();

                if visited.contains(&node.coordinate) {
                    continue;
                }

                visited.insert(node.coordinate);
                let edges = self.edges(&node);

                for e in &edges {
                    if e.node_type.is_finishing() {
                        return Some(count + 1);
                    }
                }

                next_round.extend(edges);
            }

            current_nodes.extend(next_round);
            count += 1;
        }

        None
    }

    fn in_bounds(&self, (x, y): (usize, usize)) -> bool {
        x < self.width && y < self.height
    }

    fn node(&self, (x, y): (usize, usize)) -> Option<&Node> {
        if !self.in_bounds((x, y)) {
            return None;
        }
        let n = self.nodes.iter().find(|n| {
            let (n_x, n_y) = n.coordinate;
            n_x == x && n_y == y
        });
        n
    }

    fn edges(&self, node: &Node) -> Vec<Node> {
        let current_height = node.node_type.height();
        let (x, y) = node.coordinate;
        let possible_edges: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        possible_edges
            .into_iter()
            .filter_map(|(dx, dy)| {
                let coordinate = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
                if !self.in_bounds(coordinate) {
                    return None;
                }

                let next_node = self.node(coordinate).unwrap().clone();

                if next_node.node_type.height() < current_height - 1 {
                    return None;
                }

                Some(next_node)
            })
            .collect::<Vec<Node>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn parsing() {
        let graph = Graph::new(INPUT);
        let node = graph.node((8, 8));
        let node_two = graph.node((7, 4));
        assert!(node.is_none());
        assert!(node_two.is_some());
    }

    #[test]
    fn edges() {
        let graph = Graph::new(INPUT);
        let node = graph.node((5, 2)).unwrap();
        assert_eq!(graph.edges(node).len(), 1);
    }

    #[test]
    fn bfs() {
        let graph = Graph::new(INPUT);
        let node = graph.bfs().unwrap();
        assert_eq!(node, 29);
    }
}
