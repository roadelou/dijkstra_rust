// Used for graph representation.
use std::collections::{HashMap, HashSet};

fn main() {
    // All the nodes of the graph.
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";

    // Create the hashmap which represents the network.
    let mut graph: HashMap<&str, HashMap<&str, u32>> = HashMap::new();

    // We create the HashMap of adjacent nodes for all our nodes.
    let mut a_adjacent: HashMap<&str, u32> = HashMap::new();
    let mut b_adjacent: HashMap<&str, u32> = HashMap::new();
    let mut c_adjacent: HashMap<&str, u32> = HashMap::new();
    let mut d_adjacent: HashMap<&str, u32> = HashMap::new();

    // The network is like this:
    //
    //  A -> B
    //  ^    |
    //  |    v
    //  D <- C

    a_adjacent.insert(b, 1);
    b_adjacent.insert(c, 1);
    c_adjacent.insert(d, 1);
    d_adjacent.insert(a, 1);

    // We build the final graph representation.
    graph.insert(a, a_adjacent);
    graph.insert(b, b_adjacent);
    graph.insert(c, c_adjacent);
    graph.insert(d, d_adjacent);

    // We run dijkstra's algorithm to find the shortest path between A and D.
    let path = dijkstra(graph, a, d);

    // We print the shortest path we have found, if any.
    println!(
        "{}",
        path.map(
            // The path is represented by the nodes joined with a "->".
            |node_list| format!("Path found: {}", node_list.join(" -> ")),
        ).unwrap_or(
                // If no path was found, we print an error message instead.
                "No path was found".to_string(),
            )
    )
}

// Note that the strings returned by the function are taken from the graph
// itself. The lifetime of source has to be the same as the keys in graph for
// the dijkstra_backtrack base case.
fn dijkstra<'a>(
    graph: HashMap<&'a str, HashMap<&'a str, u32>>,
    source: &'a str,
    destination: &'a str,
) -> Option<Vec<&'a str>> {
    // We create a HashMap to keep track of the progression of the algorithm.
    let mut progression = HashMap::new();
    // Used to backtrack our path at the end of the execution.
    let mut origin = HashMap::new();
    // Used to avoid infinite recursion and detect failure.
    let mut visited = HashSet::new();

    // We initialize the progression of the algorithm.
    progression.insert(source, 0);

    // We recursively perform dijkstra's algorithm.
    dijkstra_progression(
        graph,
        &mut progression,
        &mut origin,
        &mut visited,
        destination,
    ).and_then(
        // We backtrack to find the path used to reach the destination, if any.
        |_| dijkstra_backtrack(origin, source, destination),
    ) // We return the path found in the right order, if any.
}

// Recursive progression function for Dijkstra's algorithm. The string returned
// by the function is a reference to that coming from destination. A second
// lifetime is used for reference and progression, which are both mutable
// variables for the recursion. Same goes for origin.
fn dijkstra_progression<'a, 'b, 'c>(
    graph: HashMap<&str, HashMap<&'a str, u32>>,
    progression: &'b mut HashMap<&'a str, u32>,
    origin: &mut HashMap<&'a str, &'a str>,
    visited: &mut HashSet<&'b str>,
    destination: &'c str,
) -> Option<&'c str> {
    // We start by getting the next node according to the progression. We
    // propagate the get_next_node Option as there isn't much we can do about
    // it.
    let (next_node, current_progression) = get_next_node(progression, visited)?;

    if next_node == destination {
        // Base case and success condition.
        Some(destination)
    } else {
        // We get all the nodes adjacent to this one in the graph. If the node
        // doesn't exist in the graph (i.e. it is a terminal node) we use an
        // empty iterator instead. We have to borrow our rvalue because
        // graph.get returns a reference.
        for (node, cost) in graph.get(&next_node).unwrap_or(&HashMap::new()) {
            // For each adjacent node, we try to update the progression.
            progression
                .entry(*node)
                .and_modify(
                    // We update the old progression if we must.
                    |old_progression| if *old_progression > current_progression + cost {
                        // We have found a new best path, we update the source to
                        // the node and the progression.
                        origin.insert(*node, next_node);
                        // We update the progression for this node.
                        *old_progression = current_progression + cost
                    },
                )
                .or_insert_with(
                    // If there is no progression for the node (i.e. it had never
                    // been reached), we set the initial value and a first origin.
                    || {
                        origin.insert(*node, next_node);
                        current_progression + cost
                    },
                );
        }

        // We mark the current node as visited.
        visited.insert(next_node);

        // Tail recursion.
        dijkstra_progression(graph, progression, origin, visited, destination)
    }
}

// Helper function to see which node we should explore next according to
// Dijkstra's algorithm. The returned string is a reference to a key in the
// HashMap.
fn get_next_node<'a>(
    progression: &HashMap<&'a str, u32>,
    visited: &HashSet<&str>,
) -> Option<(&'a str, u32)> {
    progression
        .iter()
        .filter(
            // We filter out the nodes we have already encountered.
            |(node, _)| !visited.contains(*node),
        )
        .reduce(|(first_node, first_cost), (second_node, second_cost)| {
            // We use the reduce function to find the node with the highest priority.
            if first_cost <= second_cost {
                (first_node, first_cost)
            } else {
                (second_node, second_cost)
            }
        })
        .map(
            // If we found a valid tuple, we dereference it.
            |(node, cost)| (*node, *cost),
        )
}

// The strings in the returned vector come from the origin HashMap. We also have
// to give the same lifetime to the source as it is used in the base case.
fn dijkstra_backtrack<'a>(
    origin: HashMap<&str, &'a str>,
    source: &'a str,
    destination: &'a str,
) -> Option<Vec<&'a str>> {
    // We recursively perform the backtracking, and then return the path in the
    // right order by reversing it.
    Some(
        dijkstra_backtrack_recursive(&origin, source, destination, &mut Vec::new())?
            .into_iter()
            .rev()
            .collect(),
    )
}

// Returns a Vec<&str> representing the nodes involved in the path found by
// dijkstra's algorithm. Recursive implementation used by dijkstra_backtrack.
fn dijkstra_backtrack_recursive<'a>(
    origin: &HashMap<&str, &'a str>,
    source: &'a str,
    location: &'a str,
    path: &mut Vec<&'a str>,
) -> Option<Vec<&'a str>> {
    // Base case.
    if location == source {
        // We add the final source to the path.
        path.push(source);
        // We return the prepared path.
        Some(path.to_vec())
    } else {
        // We start by seeing how we reached the current location. This could
        // fail if the current location was never reached, returnin None.
        origin.get(&location).and_then(|&origin_node| {
            // If we found an origin_node, we push the current location to the path.
            path.push(location);
            // We recursively compute the rest of the path. This is tail
            // recursion BTW.
            dijkstra_backtrack_recursive(origin, source, origin_node, path)
        })
    }
}
