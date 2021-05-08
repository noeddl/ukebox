use std::iter::Iterator;

use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;

use crate::{Chord, ChordSequence, Distance, Semitones, Voicing, VoicingConfig};

const MAX_DIST: Semitones = 10;

/// A graph whose nodes represent chord voicings and whose edges
/// are weighted by the distances between the voicings. It is used
/// to find the (by some definition) optimal voice leading for
/// a given sequence of chords.
pub struct VoicingGraph {
    graph: Graph<Voicing, Distance>,
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
        for (l, r) in left_nodes.iter().cartesian_product(right_nodes.iter()) {
            let l_voicing = self.graph[*l];
            let r_voicing = self.graph[*r];

            let dist1 = match l {
                l if *l == self.start_node => 0,
                _ => l_voicing.semitone_distance(r_voicing),
            };

            let dist2 = match l {
                l if *l == self.start_node => 0,
                _ => l_voicing.fingering_distance(r_voicing),
            };

            // Ignore voicings that are too far away from each other.
            if dist1 <= MAX_DIST {
                self.graph.add_edge(*l, *r, Distance::new(dist1, dist2));
            }
        }
    }

    pub fn add(&mut self, chord_seq: &ChordSequence) {
        // Add edges from the start node to all the voicings of the first chord.
        let mut prev_nodes = vec![self.start_node];

        for chord in chord_seq.chords() {
            let nodes = self.add_nodes(chord);
            self.add_edges(&prev_nodes, &nodes);

            prev_nodes = nodes;
        }

        // Add edges from all the voicings of the last chord to the end node.
        for node in prev_nodes.iter() {
            self.graph
                .add_edge(*node, self.end_node, Distance::default());
        }

        // Remove unused nodes.
        let end_node = self.end_node;

        self.graph
            .retain_nodes(|g, n| g.neighbors(n).count() > 0 || n == end_node);
    }

    /// Return an iterator over the paths between the voicing nodes.
    /// The path with the lowest distance is presented first. If several paths
    /// have the same overall distance, they are further ranked by fingering
    /// distance.
    pub fn paths(
        &self,
        max_suggestions: usize,
    ) -> impl Iterator<Item = (Vec<Voicing>, Distance)> + '_ {
        let all_paths = all_simple_paths::<Vec<NodeIndex>, &Graph<Voicing, Distance>>(
            &self.graph,
            self.start_node,
            self.end_node,
            0,
            None,
        );

        // Compute the sum of th weights along a path.
        let weight_sum = |path: &Vec<NodeIndex>| -> Distance {
            path.iter()
                // Loop over (overlapping) pairs of nodes.
                .tuple_windows()
                // Fetch edge between nodes.
                .filter_map(|(n1, n2)| self.graph.find_edge(*n1, *n2))
                // Get edge weight.
                .filter_map(|e| self.graph.edge_weight(e))
                .sum()
        };

        let mut paths_with_dist = vec![];

        for path in all_paths.sorted_by_key(weight_sum).take(max_suggestions) {
            let voicing_path: Vec<_> = path
                .iter()
                .enumerate()
                // Ignore start and end node.
                .filter(|(i, _node)| *i > 0 && *i < path.len() - 1)
                .map(|(_i, node)| self.graph[*node])
                .collect();

            paths_with_dist.push((voicing_path, weight_sum(&path)))
        }

        paths_with_dist.into_iter()
    }
}
