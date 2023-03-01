// use itertools::Itertools;

trait Graph {
    type Node;
    type Index;
    fn add_node(&mut self, node: Self::Node) -> Self::Index;
    fn add_edge(&mut self, from: Self::Index, to: Self::Index);
    fn get_outgoing_indexes(&self, node: Self::Index) -> Vec<Self::Index>;
    fn index_to_node(&self, index: Self::Index) -> &Self::Node;
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Ix(usize);
// TODO: smart constructor, Edge can be created only by impl Graph - not by client code
struct Edge(Ix, Ix);

struct Graph1<N> {
    nodes: Vec<N>,
    edges: Vec<Edge>,
}

impl<N> Graph1<N> {
    fn new() -> Graph1<N> {
        Graph1 {
            nodes: vec![],
            edges: vec![],
        }
    }
}

impl<N> Graph for Graph1<N> {
    type Node = N;
    type Index = Ix;
    fn add_node(&mut self, node: N) -> Ix {
        self.nodes.push(node);
        Ix(self.nodes.len())
    }
    fn add_edge(&mut self, from: Ix, to: Ix) {
        self.edges.push(Edge(from, to))
    }
    fn get_outgoing_indexes(&self, node: Ix) -> Vec<Ix> {
        self.edges
            .iter()
            .filter(|Edge(from, to)| from == &node || to == &node)
            .map(|Edge(from, to)| if from == &node { *to } else { *from })
            //            .unique()
            .collect()
    }
    fn index_to_node(&self, index: Ix) -> &N {
        let Ix(s) = index;
        self.nodes.get(s).unwrap()
    }
}

fn main() {
    let mut g = Graph1::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    g.add_edge(n1, n2);
    g.add_edge(n2, n3);
    g.add_edge(n3, n1);
    println!(
        "{:?}",
        g.get_outgoing_indexes(n1)
            .iter()
            .map(|i| g.index_to_node(*i))
    )
}