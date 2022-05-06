use petgraph::{dot::Dot, graph::DiGraph};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let name: String = env::args().nth(1).expect("file name not provided");

    let lines = BufReader::new(
        File::open(name.clone()).unwrap_or_else(|err| panic!("error opening file: {}", err)),
    )
    .lines();

    let mut num_qudits = 0;

    // Representing a directed acyclic graph (DAG) where nodes
    // are represented using a tuple (index, dimension) and edges
    // as operation name
    let mut graph: DiGraph<u32, char> = DiGraph::new();

    let mut name_map = HashMap::new();

    for (line_no, line) in lines.enumerate() {
        match line
            .unwrap_or("".to_string())
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["qudit", q, d] => {
                let d = d
                    .parse::<u32>()
                    .unwrap_or_else(|err| panic!("error parsing line {} : {}", line_no, err));

                let node = graph.add_node(d);

                name_map.insert(q.to_string(), (node, d));

                num_qudits += 1;

                println!("{} {}", q, d);
            }
            ["h", q] | ["H", q] => {
                let (node, dim) = name_map
                    .get_mut(&q.to_string())
                    .expect(&format!("Did not define qudit {}", q));
                let end = graph.add_node(*dim);
                graph.add_edge(*node, end, 'H');
                *node = end;
            }
            ["c", q_c, q_t] | ["C", q_c, q_t] => {
                let (node_c, dim_c) = *name_map
                    .get(&q_c.to_string())
                    .expect(&format!("Did not define qudit {}", q_c));
                let (node_t, dim_t) = *name_map
                    .get(&q_t.to_string())
                    .expect(&format!("Did not define qudit {}", q_t));
                let end_c = graph.add_node(dim_c);
                let end_t = graph.add_node(dim_t);
                graph.add_edge(node_c, end_c, 'I');
                graph.add_edge(node_t, end_t, 'I');
                graph.add_edge(node_c, end_t, 'I');

                {
                    let (node, _) = name_map.get_mut(&q_c.to_string()).unwrap();
                    *node = end_c;
                }
                {
                    let (node, _) = name_map.get_mut(&q_t.to_string()).unwrap();
                    *node = end_t;
                }
            }
            _ => {}
        }
    }

    fs::write(format!("{}.dot", name), format!("{:?}", Dot::new(&graph))).expect("Can't write");

    println!("Total qudits: {}", num_qudits);
}
