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
    config: VoicingConfig,
}

impl VoicingGraph {
    pub fn new(config: VoicingConfig) -> Self {
        let mut graph = Graph::new();

        let start_node = graph.add_node(Voicing::default());
        let end_node = graph.add_node(Voicing::default());

        Self {
            graph,
            start_node,
            end_node,
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

    fn add_edges(&mut self, left_nodes: &[NodeIndex], right_nodes: &[NodeIndex]) {
        for (l, r) in left_nodes.iter().cartesian_product(right_nodes.iter()) {
            let l_voicing = self.graph[*l];
            let r_voicing = self.graph[*r];

            let distance = match l {
                l if *l == self.start_node => 0,
                _ => l_voicing.distance(r_voicing),
            };

            self.graph.add_edge(*l, *r, distance);
        }
    }

    pub fn add(&mut self, chords: &[Chord]) {
        let mut prev_nodes = vec![self.start_node];

        for chord in chords {
            let nodes = self.add_nodes(chord);

            self.add_edges(&prev_nodes, &nodes);

            prev_nodes = nodes;

            println!(
                "- {:?}, {:?}",
                self.graph.node_count(),
                self.graph.edge_count()
            );
        }

        // Add edges from all the voicings of the last chord in the sequence
        // to the end node.
        for node in prev_nodes.iter() {
            self.graph.add_edge(*node, self.end_node, 0);
        }
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
