use super::*;


/// Vertex in a triangle have a weight assigned and all Edge's directing to them
/// will have that weight.
#[derive(Clone, Copy)]
struct TriangleVertex {
    value: u8,
    vertex: Vertex,
}


/// Structure for Triangles
/// 
/// Contains a Vector of the Vertices to use
/// (only works if the len() of this is a triangle number)
/// and a directed graph.
pub struct Triangle {
    vertex_vec: Vec<TriangleVertex>,
    graph: Graph,
    base: usize,
}


impl std::fmt::Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("Values:\n");
        // Values
        for row in 0..self.base {
            let spaces = (0..(2 * (self.base - row - 1))).map(|_| ' ').collect::<String>();
            s.push_str(&format!("{}", spaces));
            for col in 0..=row {
                let tv = self.vertex_vec[Triangle::get_idx(row, col)];
                s.push_str(&format!("{}  ", display_number(tv.value, false)));
            }
            s.push_str("\n");
        }
        // Vertices
        s.push_str("\nVertices:\n");
        for row in 0..self.base {
            let spaces = (0..(2 * (self.base - row - 1))).map(|_| ' ').collect::<String>();
            s.push_str(&format!("{}", spaces));
            for col in 0..=row {
                let tv = self.vertex_vec[Triangle::get_idx(row, col)];
                s.push_str(&format!("{:?}  ", &tv.vertex));
            }
            s.push_str("\n");
        }
        // Graph
        s.push_str(&format!("\n{:?}\n", &self.graph));
        write!(f, "{}", &s)
    }
}


impl Triangle {
    pub fn get_idx(row: usize, col: usize) -> usize {
        (row * (row + 1) / 2) + col
    }

    /// Will use the given numbers to make a triangle of them, the first being the top
    pub fn from(number_slice: &[u8]) -> Triangle {
        // Calculate the base of the triangle
        let base = ((((8 * number_slice.len() + 1) as f64).sqrt()) as usize - 1) / 2;
        // Make a vector of the vertices
        let mut vertex_vec = Vec::new();
        for &number in number_slice {
            let tn = TriangleVertex {
                value: number,
                vertex: Vertex::new()
            };
            vertex_vec.push(tn);
        }
        // Make the HashMap of edges
        let mut graph_hm = HashMap::new();
        for row in 0..(base - 1)  {
            for col in 0..=row {
                let parent = vertex_vec[Triangle::get_idx(row, col)];
                // Make edges
                let mut edge_set = HashSet::new();
                let child = vertex_vec[Triangle::get_idx(row + 1, col)];
                edge_set.insert(parent.vertex.to(child.vertex, child.value));
                let child = vertex_vec[Triangle::get_idx(row + 1, col + 1)];
                edge_set.insert(parent.vertex.to(child.vertex, child.value));
                // Store them in the HashMap
                graph_hm.insert(parent.vertex, edge_set);
            }
        }
        // Make the root and goal and connect them
        let root = Vertex::new_with_id('R');
        let mut edge_set = HashSet::new();
        edge_set.insert(root.to(vertex_vec[0].vertex, vertex_vec[0].value));
        graph_hm.insert(root, edge_set);

        let goal = Vertex::new_with_id('G');
        for col in 0..base{
            let parent = vertex_vec[Triangle::get_idx(base - 1, col)].vertex;
            let edge = parent.to(goal, 0);
            graph_hm.insert(parent, HashSet::from([edge]));
        }
        graph_hm.insert(goal, HashSet::new());
        // Make the graph
        let graph = Graph { graph_hm, root, goal };
        // Make the triangle
        Triangle { vertex_vec, graph, base }
    }

    /// Will print the shortest path, and return its value.
    pub fn shortest_path(&self) -> u16 {
        let (dist, path) = self.graph.dijkstra();
        let path_set: HashSet<_> = HashSet::from_iter(path.iter().copied());
        print!("\n");
        for row in 0..self.base {
            let spaces = (0..(2 * (self.base - row - 1))).map(|_| ' ').collect::<String>();
            print!("{}", spaces);
            for col in 0..=row {
                let tv = self.vertex_vec[Triangle::get_idx(row, col)];
                print!("{}  ", display_number(tv.value, path_set.contains(&tv.vertex)));
            }
            print!("\n");
        }

        dist
    }

    /// Will print the longest path, and return its value.
    pub fn longest_path(&self) -> u16 {
        // Make alt graph
        let mut alt_graph_hm = HashMap::new();
        for (&v, edge_set) in self.graph.graph_hm.iter() {
            let alt_edge_set: HashSet<Edge> = HashSet::from_iter(
                edge_set.iter().map(|e| e.copy_big_weight())
            );
            alt_graph_hm.insert(v, alt_edge_set);
        }
        let alt_graph = Graph {
            graph_hm: alt_graph_hm,
            root: self.graph.root,
            goal: self.graph.goal
        };
        // Calculate shortest path of the alt graph
        let (_, path_set) = alt_graph.dijkstra();
        // Display the path
        let mut dist = 0;
        for row in 0..self.base {
            let spaces = (0..(2 * (self.base - row - 1))).map(|_| ' ').collect::<String>();
            print!("{}", spaces);
            for col in 0..=row {
                let tv = self.vertex_vec[Triangle::get_idx(row, col)];
                print!("{}  ", display_number(tv.value, path_set.contains(&tv.vertex)));
                // Add the values of the nodes in the path
                if path_set.contains(&tv.vertex) {
                    dist += tv.value as u16;
                }
            }
            print!("\n");
        }
        dist
    }
}
