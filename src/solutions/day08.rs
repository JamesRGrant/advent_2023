use crate::Solve;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Outgoing};
use std::collections::HashMap;

pub struct Problem {
    directions: Vec<char>,
    graph: Graph<String, char>,
    nodes: HashMap<String, NodeIndex>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let x = self.nodes.get("AAA").unwrap();
        self.graph_count_steps(*x, true)
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        let mut retval = 1;

        for node in self.graph.node_indices() {
            let key = self.graph.node_weight(node).unwrap();
            if key.chars().nth(2).unwrap() == 'A' {
                let step_cnt = self.graph_count_steps(node, false);
                retval = Problem::lcm(retval, step_cnt);
            }
        }

        retval
    }
}
impl Problem {
    // From: https://www.hackertouch.com/least-common-multiple-in-rust.html
    pub fn lcm(a: i64, b: i64) -> i64 {
        a * b / Problem::gcd(a, b)
    }

    // From: https://www.hackertouch.com/least-common-multiple-in-rust.html
    fn gcd(a: i64, b: i64) -> i64 {
        let mut max = a;
        let mut min = b;
        if b > a {
            std::mem::swap(&mut min, &mut max);
        }
        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }
            max = min;
            min = res;
        }
    }

    pub fn graph_count_steps(&self, start: NodeIndex, zzz: bool) -> i64 {
        let mut walker = start;
        let mut step_count = 0;
        let mut i = 0;

        while i <= self.directions.len() {
            let mut edges = self.graph.neighbors_directed(walker, Outgoing).detach();
            while let Some(edge) = edges.next_edge(&self.graph) {
                let w = self.graph.edge_weight(edge).unwrap();
                if *w == self.directions[i] {
                    walker = self.graph.edge_endpoints(edge).unwrap().1;
                    break;
                }
            }
            step_count += 1;

            let key = self.graph.node_weight(walker).unwrap();
            if key == "ZZZ" && zzz {
                break;
            }
            let mut flag = true;
            if key.chars().nth(2).unwrap() != 'Z' {
                flag = false;
            }
            if flag {
                break;
            }

            if i == self.directions.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
        }
        step_count
    }

    pub fn new(data: &[String]) -> Self {
        let directions: Vec<char> = data[0].chars().collect();
        let mut maps: Vec<(String, String, String)> = Vec::new();
        let mut graph = Graph::<String, char>::new();
        let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

        for line in data.iter().skip(2) {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let source = tokens[0].to_string();
            let left = tokens[2].to_string().replace(['(', ','], "");
            let right = tokens[3].to_string().replace(')', "");

            maps.push((source.clone(), left.clone(), right.clone()));
            nodes.insert(source.clone(), graph.add_node(source.clone()));
        }

        // Add the edges to the graph
        for (key, left, right) in maps {
            graph.add_edge(nodes[&key], nodes[&left], 'L');
            graph.add_edge(nodes[&key], nodes[&right], 'R');
        }

        Problem {
            directions,
            graph,
            nodes,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\08_test.txt")).p1(), 6);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }
    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\08_test2.txt")).p2(), 6);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
