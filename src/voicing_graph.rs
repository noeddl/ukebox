use itertools::Itertools;
use petgraph::algo::{all_simple_paths, astar};
use petgraph::prelude::NodeIndex;
use petgraph::Graph;

use crate::{Chord, ChordSequence, Fingering, Semitones, Voicing, VoicingConfig};

/// A graph whose nodes represent chord voicings and whose edges
/// are weighted by the distances between the voicings. It is used
/// to find the (by some definition) optimal voice leading for
/// a given sequence of chords.
pub struct VoicingGraph {
    graph: Graph<Voicing, Semitones>,
    start_node: NodeIndex,
    end_node: NodeIndex,
    config: VoicingConfig,
}

impl VoicingGraph {
    pub fn new(config: VoicingConfig) -> Self {
        let mut graph = Graph::new();

        // We need a fake start and end node for finding the best path.
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
        chord
            .voicings(self.config)
            .map(|voicing| self.graph.add_node(voicing))
            .collect()
    }

    fn add_edges(&mut self, left_nodes: &[NodeIndex], right_nodes: &[NodeIndex]) {
        let mut edge_cands = vec![];

        for (l, r) in left_nodes.iter().cartesian_product(right_nodes.iter()) {
            let l_voicing = self.graph[*l];
            let r_voicing = self.graph[*r];
            let dist = l_voicing.distance(r_voicing);

            edge_cands.push((*l, *r, dist));
        }

        let edges: Vec<_> = match edge_cands.iter().map(|(_l, _r, dist)| dist).min() {
            Some(min_dist) => edge_cands
                .iter()
                .filter(|(_l, _r, dist)| dist == min_dist)
                .collect(),
            _ => edge_cands.iter().collect(),
        };

        for (l, r, dist) in edges.iter() {
            self.graph.add_edge(*l, *r, *dist);
        }
    }

    pub fn add(&mut self, chord_seq: &ChordSequence) {
        let mut prev_nodes = vec![];

        for (i, chord) in chord_seq.chords().enumerate() {
            let nodes = self.add_nodes(chord);

            // Add edges from the start node to all the voicings of the first chord.
            if i == 0 {
                for node in nodes.iter() {
                    self.graph.add_edge(self.start_node, *node, 0);
                }
            }

            self.add_edges(&prev_nodes, &nodes);

            prev_nodes = nodes;
        }

        // Add edges from all the voicings of the last chord to the end node.
        for node in prev_nodes.iter() {
            self.graph.add_edge(*node, self.end_node, 0);
        }

        // Remove unused nodes.
        let end_node = self.end_node;

        self.graph
            .retain_nodes(|g, n| g.neighbors(n).count() > 0 || n == end_node);
    }

    pub fn update_edges(&mut self) {
        for e in self.graph.edge_indices() {
            if let Some((l, r)) = self.graph.edge_endpoints(e) {
                if l != self.start_node && r != self.end_node {
                    let l_voicing = self.graph[l];
                    let r_voicing = self.graph[r];

                    let l_fingering = Fingering::from(l_voicing);
                    let r_fingering = Fingering::from(r_voicing);

                    let dist = l_fingering.distance(r_fingering);

                    self.graph.update_edge(l, r, dist);
                }
            }
        }
    }

    pub fn find_best_path(&self) -> Option<Vec<Voicing>> {
        // Find the best path through the graph.
        let path_option = astar(
            &self.graph,
            self.start_node,
            |finish| finish == self.end_node,
            |e| *e.weight(),
            |_| 0,
        );

        // Map the nodes in the path to voicings.
        if let Some((_weight, path)) = path_option {
            let voicings: Vec<Voicing> = path
                .iter()
                .enumerate()
                // Ignore start and end node.
                .filter(|(i, _node)| *i > 0 && *i < path.len() - 1)
                .map(|(_i, node)| self.graph[*node])
                .collect();

            return Some(voicings);
        };

        None
    }

    pub fn iter_paths(&self) {
        // -> impl Iterator + '_ {

        let all_paths = all_simple_paths::<Vec<NodeIndex>, &Graph<Voicing, Semitones>>(
            &self.graph,
            self.start_node,
            self.end_node,
            0,
            None,
        );

        let weight_sum = |path: &Vec<NodeIndex>| -> u8 {
            path.iter()
                // Loop over (overlapping) pairs of nodes.
                .tuple_windows()
                // Fetch edge between nodes.
                .filter_map(|(n1, n2)| self.graph.find_edge(*n1, *n2))
                // Get edge weight.
                .filter_map(|e| self.graph.edge_weight(e))
                .sum()
        };

        // Split into an iterator pair that both yield all elements from the original iterator.
        let (all_paths1, all_paths2) = all_paths.tee();

        let min_weight = all_paths1.map(|p| weight_sum(&p)).min().unwrap();

        let paths: Vec<Vec<NodeIndex>> = all_paths2
            .filter(|p| weight_sum(&p) == min_weight)
            .collect();

        for path in paths {
            for voicing in path.iter().map(|n| self.graph[*n]) {
                println!("{:?} {:?}", min_weight, voicing);
            }
            println!("----");
        }
    }
}
