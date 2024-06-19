// Causes compiler error: "recursive without indirection"
// struct NodeBad {
//     data: i32,
//     next: Option<NodeBad>
// }

struct Node {
    data: i32,
    next: Option<Box<Node>>
}

impl Node {
    fn new(data: i32) -> Node {
        Node {data: data, next: None}
    }

    fn append(&mut self, data: i32) {        
        match self.next {
            None => {
                self.next = Some(Box::new(Node::new(data)));
            },
            Some(ref mut boxed_next_node) => {
                boxed_next_node.append(data);
            }
        }
    }

    fn display(&self) {
        println!("{} ", self.data);
        match self.next {
            None => {    
                println!("[END]");
            },
            Some(ref boxed_next_node) => {
                boxed_next_node.display();
            },
        }
    }
}

struct Chain {
    head: Option<Box<Node>>
}

impl Chain {

    fn new() -> Chain {
        Chain {head: None} 
    }

    fn insert(&mut self, data: i32) {
        match self.head {
            None => {
                self.head = Some(Box::new(Node::new(data)));
            },
            Some(ref mut boxed_head_node) => {
                boxed_head_node.append(data);
            }
        }
    } 

    fn display(&self) {
        match self.head {
            None => {
                println!("Empty chain");
            },
            Some(ref boxed_head_node) => {
                boxed_head_node.display()
            }
        }
    }
}

pub fn run() {
    let mut chain = Chain::new();
    chain.insert(100);
    chain.insert(200);
    chain.insert(300);
    chain.display();
}    