use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::Graph;

use crate::{Chord, Voicing, VoicingConfig};

pub fn dist() {
    let config = VoicingConfig::default();
    let chord_c = Chord::from_str("C").unwrap();
    let chord_g = Chord::from_str("F").unwrap();

    let mut graph = Graph::new();

    let mut nodes2voicings = HashMap::new();

    let mut nodes_c = vec![];

    for voicing in chord_c.voicings(config) {
        let c = graph.add_node(voicing);
        nodes_c.push(c);
        nodes2voicings.insert(c, voicing);
    }

    println!("{:?}", nodes_c.len());
    println!("{:?}, {:?}", graph.node_count(), graph.edge_count());

    let mut nodes_g = vec![];

    for voicing in chord_g.voicings(config) {
        let g = graph.add_node(voicing);
        nodes_g.push(g);
        nodes2voicings.insert(g, voicing);
    }

    println!("{:?}, {:?}", graph.node_count(), graph.edge_count());

    for (c, g) in nodes_c.iter().cartesian_product(nodes_g.iter()) {
        let chord_c = nodes2voicings.get(c).unwrap();
        let chord_g = nodes2voicings.get(g).unwrap();
        graph.add_edge(*c, *g, chord_c.distance(*chord_g));
    }

    println!("{:?}, {:?}", graph.node_count(), graph.edge_count());

    let start = graph.add_node(Voicing::default());

    for c in nodes_c.iter() {
        graph.add_edge(start, *c, 0);
    }

    println!("{:?}, {:?}", graph.node_count(), graph.edge_count());

    let end = graph.add_node(Voicing::default());

    for g in nodes_g.iter() {
        graph.add_edge(*g, end, 0);
    }

    println!("{:?}, {:?}", graph.node_count(), graph.edge_count());

    if let Some((weight, path)) = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| *e.weight(),
        |_| 0,
    ) {
        println!("{:?} {:?}", weight, path);

        for (i, node) in path.iter().enumerate() {
            if i > 0 && i < path.len() - 1 {
                println!("{:?}", nodes2voicings.get(node).unwrap());
            }
        }
    };
}
