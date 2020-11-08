use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    val: &'a str,
    weight: u32,
    children: Option<Vec<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn add_data(&mut self, children: Option<Vec<Node<'a>>>) {
        self.children = children;
    }

    pub fn traverse(&self) {
        println!("{:?}", self);
        match &self.children {
            Some(x) => {
                for child in x.iter() {
                    child.traverse();
                }
            }
            None => {}
        }
    }

    // find total count of all candidates - reduce weight to vote count with order of ranking
    pub fn get_count(&self, ratio: u32) -> u32 {
        let mut count: u32 = 0;

        match &self.children {
            Some(x) => {
                for child in x.iter() {
                    count = child.weight * ratio + child.get_count(ratio / 2);
                }
            }
            None => {}
        }
        count
    }

    // calculate total count of all candidates and return winner
    pub fn get_winner(&self) -> String {
        let mut count_map: HashMap<String, u32> = HashMap::new();

        match &self.children {
            Some(x) => {
                for child in x.iter() {
                    let v = child.get_count(1) + child.weight;
                    count_map.insert(child.val.to_string(), v);
                }
            }
            None => {}
        }

        let res = count_map.iter().max_by_key(|entry| entry.1).unwrap();

        res.0.to_string()
    }
}

fn main() {
    // Dirty data for testing
    // TODO : Load from a large list of ballots to Node
    let mut new_node = Node {
        val: "Total",
        weight: 10,
        children: None,
    };

    let child_node_ac = Node {
        val: "C",
        weight: 1,
        children: None,
    };
    let child_node_ab = Node {
        val: "B",
        weight: 1,
        children: None,
    };

    let child_node_ca = Node {
        val: "A",
        weight: 2,
        children: None,
    };

    let child_node_a = Node {
        val: "A",
        weight: 5,
        children: Some(vec![child_node_ab, child_node_ac]),
    };
    let child_node_b = Node {
        val: "B",
        weight: 3,
        children: None,
    };
    let child_node_c = Node {
        val: "C",
        weight: 2,
        children: Some(vec![child_node_ca]),
    };
    let child_refs: Vec<Node> = vec![child_node_a, child_node_b, child_node_c];

    new_node.add_data(Some(child_refs));
    new_node.traverse();
    println!("{}", new_node.get_winner());
}
