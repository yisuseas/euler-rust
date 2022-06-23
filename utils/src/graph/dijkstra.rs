use super::*;

impl Graph {
    /// ## Dijkstra Algorithm to find the shortest path
    /// ### Returns: 
    /// The path's lenght and a vector containing the vertices
    /// of said path in order
    /// ### Algorithm:
    /// 1. Assign the max distance posible for all nodes to the root
    /// 2. Add all nodes to a priority queue
    /// 3. Set the distance from the root node to itself to 0
    /// 4. Loop on the queue as long as its not empty:
    ///     1. Choose the node with the minimum distance from root
    ///     2. Remove the current chosen node from the queue, will remove root on first iteration
    ///     3. If the current chosen node is the goal, then return it.
    ///     4. Iterate on each child of the current node:
    ///         1. If child is not in the queue, skip this iteration
    ///         2. Get the child's distance to root with this path
    ///         3. If its shorter than the one stored, change that.
    /// 5. If the queue is empty, the goal was not found.
    pub fn dijkstra(&self) -> (u16, Vec<Vertex>) {
        let mut dis_from_root = HashMap::new();
        let mut best_parent = HashMap::new();
        let mut queue = HashSet::new();      
        for &v in self.graph_hm.keys() {
            // Asign the max distance posible for all nodes to the root
            dis_from_root.insert(v, u16::MAX);
            // Asign the best parent of everyone
            best_parent.insert(v, None);
            // Add all nodes to a priority queue
            queue.insert(v);
        }
        // Set the distance from the root node to itself to 0
        if let Some(d) = dis_from_root.get_mut(&self.root) {
            *d = 0;
        }
        //Loop on the queue as long as its not empty
        while queue.len() > 0 {
            // Choose the node with the minimum distance from root
            let mut current = self.goal;
            let mut shortest_dis = dis_from_root[&current];
            for v in &queue {
                if dis_from_root[v] < shortest_dis {
                    shortest_dis = dis_from_root[v];
                    current = *v;
                }
            }
            // Remove the current chosen node from the queue
            queue.remove(&current);
            // If the current chosen node is the goal, then return it.
            if current == self.goal {
                // Find the shortest path
                let mut best_path = vec![current];
                let mut last = current;
                loop {
                    match best_parent[&last] {
                        Some(parent) => {
                            best_path.push(parent);
                            last = parent;
                        },
                        None => break
                    }
                }
                best_path.reverse();
                return (dis_from_root[&current], best_path);
            }
            // For every child of the current node...
            self.graph_hm[&current]
                .iter()
                .map(|e| (e.tail(), e.weight as u16))
                .for_each(|(child, child_to_current)| {
                    // If child is not already in the queue, skip this iteration
                    if queue.contains(&child) {
                        // Get the child's distance from root with this paht
                        let temp = dis_from_root[&current] + child_to_current;
                        // If its shorter than the one stored, change that
                        if temp < dis_from_root[&child] {
                            if let Some(child_to_root) = dis_from_root.get_mut(&child) {
                                *child_to_root = temp;
                            }
                            if let Some(parent) = best_parent.get_mut(&child) {
                                *parent = Some(current);
                            }
                        }
                    }
            });
        }
        // If queue is empty, then the goal was not found
        panic!("Queue got empty");
    }
}
