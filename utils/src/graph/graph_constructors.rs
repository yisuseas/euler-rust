use super::*;

impl Graph {
    /// https://miro.medium.com/max/1400/1*t8fjmUG8aIURU_bz8MMqOQ.png
    pub fn test_graph() -> Graph {
        let a = Vertex::new_with_id('a');
        let b = Vertex::new_with_id('b');
        let c = Vertex::new_with_id('c');
        let d = Vertex::new_with_id('d');
        let e = Vertex::new_with_id('e');
        let f = Vertex::new_with_id('f');
        let g = Vertex::new_with_id('g');

        let ab = a.to(b, 4);
        let ac = a.to(c, 1);
        let bd = b.to(d, 3);
        let be = b.to(e, 8);
        let cd = c.to(d, 2);
        let cf = c.to(f, 6);
        let de = d.to(e, 4);
        let eg = e.to(g, 2);
        let fg = f.to(g, 8);

        let mut graph = Graph {
            graph_hm: HashMap::from([
                (a, HashSet::from([ab, ac])),
                (b, HashSet::from([bd, be])),
                (c, HashSet::from([cd, cf])),
                (d, HashSet::from([de])),
                (e, HashSet::from([eg])),
                (f, HashSet::from([fg])),
                (g, HashSet::new()),
            ]),
            root: a,
            goal: g
        };
        graph.bidirect_edges();

        graph
    }


    pub fn computerphile() -> Graph {
        let a = Vertex::new_with_id('a');
        let b = Vertex::new_with_id('b');
        let c = Vertex::new_with_id('c');
        let d = Vertex::new_with_id('d');
        let e = Vertex::new_with_id('e');
        let f = Vertex::new_with_id('f');
        let g = Vertex::new_with_id('g');
        let h = Vertex::new_with_id('h');
        let i = Vertex::new_with_id('i');
        let j = Vertex::new_with_id('j');
        let k = Vertex::new_with_id('k');
        let l = Vertex::new_with_id('l');
        let r = Vertex::new_with_id('r');

        Graph::bidirected_from(&[
            (r, &[(a, 7), (b, 2), (c, 3)]),
            (a, &[(b, 3), (d, 4)]),
            (b, &[(d, 4), (h, 1)]),
            (c, &[(l, 2)]),
            (d, &[(f, 5)]),
            (e, &[(h, 2), (g, 2)]),
            (f, &[(h, 3)]),
            (h, &[]),
            (i, &[(k, 4), (l, 4)]),
            (j, &[(k, 4), (l, 4)]),
            (k, &[(g, 5)]),
            (l, &[]),
            (g, &[])
        ])
    }

    
    /// ██████░░████████████<br>
    /// ██░░░░░░░░░░░░░░░░██<br>
    /// ██░░████████░░██░░██<br>
    /// ██░░░░░░██░░░░██░░██<br>
    /// ██░░██░░██░░████░░██<br>
    /// ██░░░░░░░░░░██░░░░██<br>
    /// ██████████░░████████<br>
    /// ██░░██░░██░░████░░██<br>
    /// ██░░░░░░░░░░░░░░░░██<br>
    /// ██████████████░░████<br>
    pub fn small_maze() -> Graph {
        let root = Vertex::new_with_id('R');
        let goal = Vertex::new_with_id('G');
        
        let a = Vertex::new_with_id('a');
        let b = Vertex::new_with_id('b');
        let c = Vertex::new_with_id('c');
        let d = Vertex::new_with_id('d');
        let e = Vertex::new_with_id('e');
        let f = Vertex::new_with_id('f');
        let g = Vertex::new_with_id('g');
        let h = Vertex::new_with_id('h');
        let i = Vertex::new_with_id('i');
        let j = Vertex::new_with_id('j');
        let k = Vertex::new_with_id('k');
        let l = Vertex::new_with_id('l');
        let m = Vertex::new_with_id('m');
        let n = Vertex::new_with_id('n');
        let o = Vertex::new_with_id('o');
        let p = Vertex::new_with_id('p');
        let q = Vertex::new_with_id('q');
        let r = Vertex::new_with_id('r');
        let s = Vertex::new_with_id('s');
        let t = Vertex::new_with_id('t');
        let u = Vertex::new_with_id('u');

        Graph::bidirected_from(&[
            (root, &[(b, 1)]),
            (a, &[(b, 2), (e, 2)]),
            (b, &[(c, 3)]),
            (c, &[(d, 2), (h, 2)]),
            (d, &[(m, 4)]),
            (e, &[(f, 2), (i, 2)]),
            (f, &[(j, 2)]),
            (g, &[(h, 1), (k, 2)]),
            (h, &[]),
            (i, &[(j, 2)]),
            (j, &[(k, 2)]),
            (k, &[(s, 2)]),
            (l, &[(m, 1)]),
            (m, &[]),
            (n, &[(q, 1)]),
            (o, &[(r, 1)]),
            (p, &[(u, 1)]),
            (q, &[(r, 2)]),
            (r, &[(s, 2)]),
            (s, &[(t, 2)]),
            (t, &[(u, 1), (goal, 1)]),
            (u, &[]),
            (goal, &[])
        ])
    }

    ///    R           R
    ///    3           a
    ///   7 4         b c
    ///  2 4 6       d e f
    /// 8 5 9 3     g h i j
    ///    G           G
    pub fn small_triangle() -> Graph {
        let root = Vertex::new_with_id('R');
        let goal = Vertex::new_with_id('G');
        
        let a = Vertex::new_with_id('a');
        let b = Vertex::new_with_id('b');
        let c = Vertex::new_with_id('c');
        let d = Vertex::new_with_id('d');
        let e = Vertex::new_with_id('e');
        let f = Vertex::new_with_id('f');
        let g = Vertex::new_with_id('g');
        let h = Vertex::new_with_id('h');
        let i = Vertex::new_with_id('i');
        let j = Vertex::new_with_id('j');

        Graph::from(&[
            (root, &[(a, 3)]),
            (a, &[(b, 7), (c, 4)]),
            (b, &[(d, 2), (e, 4)]),
            (c, &[(e, 4), (f, 6)]),
            (d, &[(g, 8), (h, 5)]),
            (e, &[(h, 5), (i, 9)]),
            (f, &[(i, 9), (j, 3)]),
            (g, &[(goal, 0)]),
            (h, &[(goal, 0)]),
            (i, &[(goal, 0)]),
            (j, &[(goal, 0)]),
            (goal, &[]),
        ])
    }
}
