use day_12::Graph;

fn main() {
    let graph = Graph::new(include_str!("input.txt"));
    let len = graph.bfs().unwrap();
    println!("{}", len);
}
