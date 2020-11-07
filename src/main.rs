#[derive(Debug, Clone)]
pub struct Node<'a> {
    val : &'a str,
    weight : u32,
    children : Option<Vec<Node<'a>>>
}

impl <'a> Node <'a> {
    pub fn add_data(&mut self, children : Option<Vec<Node<'a>>>){
        self.children = children;
    }

    pub fn traverse(&self){
        println!("{:?}", self);
        match &self.children {
            Some(x) => {
                for child in x.iter(){
                    child.traverse();
                }
            }
            None => {}
        }
    }
}


fn main() {
    // Dirty data for testing
    // TODO : Load from a large list of ballots to Node
    let mut new_node = Node{val : "Total", weight : 10, children : None};

    let child_node_ac = Node{val : "AC", weight : 1, children : None};
    let child_node_ab = Node{val : "AB", weight : 1, children : None};

    let child_node_ca = Node{val : "CA", weight : 2, children : None};

    let child_node_a = Node{val : "A", weight : 4, children : Some(vec![child_node_ab, child_node_ac])};
    let child_node_b = Node{val : "B", weight : 4, children : None};
    let child_node_c = Node{val : "C", weight : 4, children : Some(vec![child_node_ca])};
    let child_refs : Vec<Node> = vec![child_node_a, child_node_b, child_node_c];

    new_node.add_data(Some(child_refs));
    new_node.traverse();

    //get_winner(&new_node);

}
