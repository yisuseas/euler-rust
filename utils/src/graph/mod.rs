//! Contains Structs usefull to work with Graph Theory

use rand::distributions::{Distribution, Uniform};
use std::collections::{HashMap, HashSet};

pub fn display_number(number: u8, enfasis: bool) -> String {
  format!(
    "{}{}{}{}",
    if enfasis { "\x1b[31m" } else { "" },
    if number < 10 { " " } else { "" },
    number,
    if enfasis { "\x1b[0m" } else { "" },
  )
}

/// Vertex Struct
///
/// All vertices have an unique id
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex {
  id: [char; 8],
}

impl std::fmt::Debug for Vertex {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s: String = self.id[..3].iter().collect();
    write!(f, "{}", s)
  }
}

impl Vertex {
  /// Will return a new Vertex with a randomly generated id
  pub fn new() -> Vertex {
    let between = Uniform::from('A'..'Z');
    let mut rng = rand::thread_rng();

    let mut char_list = ['A'; 8];
    for ch in char_list.iter_mut() {
      *ch = between.sample(&mut rng);
    }
    Vertex { id: char_list }
  }

  /// Will return a new Vertex with all characters initalized as the given __ch__
  pub fn new_with_id(ch: char) -> Vertex {
    Vertex { id: [ch; 8] }
  }

  /// Will return an Edge between self and a given Vertex, with a given weight.
  pub fn to(&self, other: Vertex, weight: u8) -> Edge {
    Edge {
      from: *self,
      to: other,
      weight,
    }
  }
}

/// Edge Struct
///
/// Contains two Vertices and a weight
///
/// All Edges are directional, for adirectional graphs consider using
/// inversed edges
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
  from: Vertex,
  to: Vertex,
  pub weight: u8,
}

impl std::fmt::Debug for Edge {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}--> {:?}", &self.weight, &self.to)
  }
}

impl Edge {
  /// Getter for the __to__ value of the Edge
  pub fn tail(&self) -> Vertex {
    self.to
  }

  /// Will return a new inversed edge.
  fn inverse(e: &Edge) -> Edge {
    Edge {
      from: e.to,
      to: e.from,
      weight: e.weight,
    }
  }

  /// Will return a copy of the Edge with a weight of the maximum possible value - the original weight
  ///
  /// Usefull for finding the longest path
  pub fn copy_big_weight(&self) -> Edge {
    Edge {
      from: self.from,
      to: self.to,
      weight: u8::MAX - self.weight,
    }
  }
}

/// Graph Struct
///
/// Contains a HashMap with key: Vertex and value: a HashSet of the Edges that start from it.
///
/// Will also store the vertices to use as root and goal,
/// Note that they also need to be in the HashMap, even if not connected
#[derive(Clone, PartialEq, Eq)]
pub struct Graph {
  pub graph_hm: HashMap<Vertex, HashSet<Edge>>,
  pub root: Vertex,
  pub goal: Vertex,
}

impl std::fmt::Debug for Graph {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = String::from("Graph:\n");
    for (k, v) in self.graph_hm.iter() {
      s.push_str(&format!("    {:?}\n", &k));
      for n in v {
        s.push_str(&format!("     +--{:?}\n", &n));
      }
      s.push_str("\n");
    }
    write!(f, "{}", s)
  }
}

impl Graph {
  /// Will make a graph, using the first Vertex given as the root
  /// and the last one as the goal
  pub fn from(input: &[(Vertex, &[(Vertex, u8)])]) -> Graph {
    let mut ghm = HashMap::new();
    for &(parent, neighbors_slice) in input {
      let mut edge_set = HashSet::new();
      for &(neighbor, dist) in neighbors_slice.iter() {
        edge_set.insert(parent.to(neighbor, dist));
      }
      ghm.insert(parent, edge_set);
    }
    Graph {
      graph_hm: ghm,
      root: input.first().unwrap().0,
      goal: input.last().unwrap().0,
    }
  }

  /// Usefull for making adirectional graphs
  pub fn bidirect_edges(&mut self) {
    // Set the inverse HashMap
    let mut inv_hm: HashMap<Vertex, HashSet<Edge>> = HashMap::new();
    for &v in self.graph_hm.keys() {
      inv_hm.insert(v, HashSet::new());
    }
    self
      .graph_hm
      .values()
      .flat_map(|edge_set| edge_set.iter())
      .for_each(|&e| {
        let e_tail = e.tail();
        let e_inv = Edge::inverse(&e);
        if let Some(inv_set) = inv_hm.get_mut(&e_tail) {
          inv_set.insert(e_inv);
        }
      });
    // Add key-value pairs from inverse HashMap to the original
    for (v, inv_set) in inv_hm.iter() {
      if let Some(original_set) = self.graph_hm.get_mut(v) {
        for &e in inv_set.iter() {
          original_set.insert(e);
        }
      }
    }
  }

  /// Usefull for making adirectional graphs
  pub fn bidirected_from(input: &[(Vertex, &[(Vertex, u8)])]) -> Graph {
    let mut g = Graph::from(input);
    g.bidirect_edges();

    g
  }
}

mod dijkstra;
mod graph_constructors;
mod triangle;
pub use triangle::Triangle;

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unique_vertices() {
    let a = Vertex::new();
    let b = Vertex::new();
    assert_ne!(a, b);
  }

  #[test]
  fn graph_constructor_works() {
    let a = Vertex::new_with_id('a');
    let b = Vertex::new_with_id('b');
    let c = Vertex::new_with_id('c');
    let d = Vertex::new_with_id('d');
    let e = Vertex::new_with_id('e');
    let f = Vertex::new_with_id('f');
    let g = Vertex::new_with_id('g');

    let graph_0 = Graph::test_graph();
    let graph_1 = Graph::bidirected_from(&[
      (a, &[(b, 4), (c, 1)]),
      (b, &[(d, 3), (e, 8)]),
      (c, &[(d, 2), (f, 6)]),
      (d, &[(e, 4)]),
      (e, &[(g, 2)]),
      (f, &[(g, 8)]),
      (g, &[]),
    ]);

    assert_eq!(graph_0, graph_1);
  }

  // Triangle

  #[test]
  fn triangle_shortest() {
    let test_numbers = [3, 7, 4, 2, 4, 6, 8, 5, 9, 3];
    let t = Triangle::from(&test_numbers);
    assert_eq!(16, t.shortest_path());
  }

  #[test]
  fn triangle_longest() {
    let test_numbers = [3, 7, 4, 2, 4, 6, 8, 5, 9, 3];
    let t = Triangle::from(&test_numbers);
    assert_eq!(23, t.longest_path());
  }
}
