use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Vertex{
    name: String
}

impl Vertex {
    fn new(name: String) -> Self {
        Vertex {
            name
        }
    }

}

#[derive(Debug, Default)]
struct Graph {
    vertices: HashMap<Vertex, Vec<Vertex>>
}

impl Graph{
    fn new(vertices: HashMap<Vertex, Vec<Vertex>>) -> Self {
        Graph {
            vertices
        }
    }
    fn add_edge(&mut self, vertex1: Vertex, vertex2: Vertex) {
        match self.vertices.get(&vertex1) {
            None => {
                self.vertices.insert(vertex1, vec![vertex2]);
            }
            Some(edge) => {
            }
        }
    }
}


fn main() {
    let mut graph: Graph = Graph::new(HashMap::new());
    println!("{:?}", graph);
    let vertex1 = Vertex::new(String::from("v1"));
    let vertex2 = Vertex::new(String::from("v2"));

    graph.add_edge(vertex1, vertex2);
    println!("{:?}", graph);
}
