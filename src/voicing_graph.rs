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
    prev_nodes: Vec<NodeIndex>,
    config: VoicingConfig,
}

impl VoicingGraph {
    pub fn new(config: VoicingConfig) -> Self {
        let mut graph = Graph::new();

        let start_node = graph.add_node(Voicing::default());
        let end_node = graph.add_node(Voicing::default());
        let prev_nodes = vec![start_node];

        Self {
            graph,
            start_node,
            end_node,
            prev_nodes,
            config,
        }
    }

    fn add_nodes(&mut self, chord: &Chord) -> Vec<NodeIndex> {
        let mut nodes = vec![];

        for voicing in chord.voicings(self.config) {
            let node = self.graph.add_node(voicing);
            nodes.push(node);
        }

        nodes
    }

    /// Add edges from all the voicings of the last chord in the sequence
    /// to the end node.
    fn finalize(&mut self) {
        for node in self.prev_nodes.iter() {
            self.graph.add_edge(*node, self.end_node, 0);
        }

        println!(
            "- {:?}, {:?}",
            self.graph.node_count(),
            self.graph.edge_count()
        );
    }

    pub fn add(&mut self, chords: &[Chord]) {
        for chord in chords {
            let nodes = self.add_nodes(chord);

            for (p, n) in self.prev_nodes.iter().cartesian_product(nodes.iter()) {
                let p_chord = self.graph[*p];
                let chord = self.graph[*n];

                let distance = match p {
                    p if *p == self.start_node => 0,
                    _ => p_chord.distance(chord),
                };

                self.graph.add_edge(*p, *n, distance);
            }

            self.prev_nodes = nodes;

            println!(
                "- {:?}, {:?}",
                self.graph.node_count(),
                self.graph.edge_count()
            );
        }

        self.finalize();
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
    let chord3 = Chord::from_str("G").unwrap();

    let chords = vec![chord1, chord2, chord3];
    let config = VoicingConfig::default();

    let mut voicing_graph = VoicingGraph::new(config);
    voicing_graph.add(&chords);
    voicing_graph.find_best_path();
}
