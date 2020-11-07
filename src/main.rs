#[derive(Debug)]
pub struct Node<'a> {
    val : &'a str,
    weight : u32,
    children : Option<Vec<Node<'a>>>
}

impl <'a> Node <'a> {
    pub fn add_data(&mut self, children : Option<Vec<Node<'a>>>){
        self.children = children;
    }
    
    pub fn get_winner(self) -> String {
        "test".to_string()
    }
}

fn main() {
    let mut new_node = Node{val : "Total", weight : 10, children : None};
    println!("{:?}", new_node);
    let child_node = Node{val : "B", weight : 4, children : None};
    let child_refs : Vec<Node> = vec![child_node];

    new_node.add_data(Some(child_refs));

    println!("{:?}", new_node);

}
