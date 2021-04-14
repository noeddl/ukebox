use std::str::FromStr;

use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;

use crate::{Chord, Voicing, VoicingConfig};

pub struct VoicingGraph {
    graph: Graph<Voicing, u8>,
    start_node: NodeIndex,
    end_node: NodeIndex,
    node_sets: Vec<Vec<NodeIndex>>,
    config: VoicingConfig,
}

impl VoicingGraph {
    pub fn new(config: VoicingConfig) -> Self {
        let mut graph = Graph::new();

        let mut node_sets = vec![];
        let start_node = graph.add_node(Voicing::default());
        let end_node = graph.add_node(Voicing::default());
        node_sets.push(vec![start_node]);

        Self {
            graph,
            start_node,
            end_node,
            node_sets,
            config,
        }
    }

    pub fn add(&mut self, chord: Chord) {
        let mut node_set = vec![];

        for voicing in chord.voicings(self.config) {
            let node = self.graph.add_node(voicing);
            node_set.push(node);
        }

        self.node_sets.push(node_set);

        let index = self.node_sets.len() - 1;
        let nodes = self.node_sets.get(index).unwrap();
        let prev_nodes = self.node_sets.get(index - 1).unwrap();

        for (p, n) in prev_nodes.iter().cartesian_product(nodes.iter()) {
            let p_chord = self.graph[*p];
            let chord = self.graph[*n];

            let distance = match p {
                p if *p == self.start_node => 0,
                _ => p_chord.distance(chord),
            };

            self.graph.add_edge(*p, *n, distance);
        }

        println!(
            "- {:?}, {:?}",
            self.graph.node_count(),
            self.graph.edge_count()
        );
    }

    pub fn finalize(&mut self) {
        let index = self.node_sets.len() - 1;
        let nodes = self.node_sets.get(index).unwrap();

        for g in nodes.iter() {
            self.graph.add_edge(*g, self.end_node, 0);
        }

        println!(
            "- {:?}, {:?}",
            self.graph.node_count(),
            self.graph.edge_count()
        );
    }

    pub fn find_best_path(&self) {
        if let Some((weight, path)) = astar(
            &self.graph,
            self.start_node,
            |finish| finish == self.end_node,
            |e| *e.weight(),
            |_| 0,
        ) {
            println!("{:?} {:?}", weight, path);

            for (i, node) in path.iter().enumerate() {
                if i > 0 && i < path.len() - 1 {
                    println!("{:?}", self.graph[*node]);
                }
            }
        };
    }
}

pub fn dist() {
    let chord1 = Chord::from_str("C").unwrap();
    let chord2 = Chord::from_str("F").unwrap();

    let chords = vec![chord1, chord2];
    let config = VoicingConfig::default();

    let mut voicing_graph = VoicingGraph::new(config);

    for chord in chords {
        voicing_graph.add(chord);
    }

    voicing_graph.finalize();
    voicing_graph.find_best_path();
}
