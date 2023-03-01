use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::rc::Weak;

struct Node<N> {
    payload: N,
    next: Vec<Weak<RefCell<Node<N>>>>,
}

struct Graph<N> {
    owned_nodes: Vec<Rc<RefCell<Node<N>>>>,
}

impl<N: Debug> Graph<N> {
    fn new() -> Graph<N> {
        Graph {
            owned_nodes: vec![],
        }
    }
    fn create_node(&mut self, payload: N) -> Weak<RefCell<Node<N>>> {
        let node = Node {
            payload,
            next: vec![],
        };
        let rc = Rc::new(RefCell::new(node));
        let rc_weak = Rc::downgrade(&rc);
        self.owned_nodes.push(rc);
        rc_weak
    }
    fn add_edge(&mut self, from: Weak<RefCell<Node<N>>>, to: Weak<RefCell<Node<N>>>) {
        from.upgrade().unwrap().borrow_mut().next.push(to)
    }
    fn print_next_nodes_payloads(from: Weak<RefCell<Node<N>>>) {
        for v in from.upgrade().unwrap().borrow().next.iter() {
            println!("{:?}", v.upgrade().unwrap().borrow().payload);
        }
    }
    fn update_next_nodes(from: &mut Weak<RefCell<Node<N>>>, f: &dyn Fn(&Weak<RefCell<Node<N>>>)) {
        for v in from.upgrade().unwrap().borrow_mut().next.iter() {
            f(v);
        }
    }
}

fn main() {
    let mut g = Graph::new();
    let n1 = g.create_node(1);
    let n2 = g.create_node(2);
    let n3 = g.create_node(3);
    g.add_edge(n1.clone(), n2.clone()); // this only clones the pointer
    g.add_edge(n2.clone(), n3.clone());
    g.add_edge(n3.clone(), n1.clone());

    Graph::print_next_nodes_payloads(n1);
}