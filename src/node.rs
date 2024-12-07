use std::fmt;
use std::fmt::Write;

#[derive(Debug)]
pub struct Node {
    id: u32,
    crd: Vec<f64>,
    net_connections: Vec<u32>,
    dual_connections: Vec<u32>,
}

impl Node {
    // Constructor
    pub fn new(
        id: u32,
        crd: Vec<f64>,
        net_connections: Vec<u32>,
        dual_connections: Vec<u32>,
    ) -> Self {
        return Node {
            id,
            crd,
            net_connections,
            dual_connections,
        };
    }

    // Convert the node to a string
    fn to_string(&self) -> String {
        // Estimate the capacity needed for the string
        let mut str = String::with_capacity(100);

        // Write the initial part of the string
        write!(
            &mut str,
            "Node {} at {}, {} with neighbours: ",
            self.id, self.crd[0], self.crd[1]
        )
        .unwrap();

        // Append net connections
        for &connected_node in &self.net_connections {
            write!(&mut str, "{} ", connected_node).unwrap();
        }

        // Append dual connections
        str.push_str("and ring neighbours: ");
        for &connected_node in &self.dual_connections {
            write!(&mut str, "{} ", connected_node).unwrap();
        }
        return str;
    }
}

// Implement the Display trait for Node to use the to_string method
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
