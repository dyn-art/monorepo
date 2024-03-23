use crate::glyph::Glyph;
use std::ops::Range;

/// An iterator over glyph clusters within a slice of glyphs.
///
/// This iterator groups adjacent glyphs based on their starting position (byte index),
/// considering glyphs with the same `start` value as part of the same cluster.
/// It's particularly useful for processing text where multiple glyphs
/// contribute to a single visual character (grapheme) or are otherwise logically grouped.
///
/// # Example
///
/// Given glyphs with starting positions like: 0, 2, 2, 2, 3, 4, 4, 5, 5,
/// the iterator will produce clusters with indices: [0, 1], [1, 4], [4, 5], [5, 7], [7, 9]
struct GlyphClusters<'a> {
    data: &'a [Glyph],
    idx: usize,
}

impl<'a> GlyphClusters<'a> {
    fn new(data: &'a [Glyph]) -> Self {
        GlyphClusters { data, idx: 0 }
    }
}

impl<'a> Iterator for GlyphClusters<'a> {
    type Item = (Range<usize>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.data.len() {
            return None;
        }

        let start = self.idx;
        let cluster_start = self.data[self.idx].range.start;

        // Iterate through the glyphs, incrementing `self.idx` for each glyph
        // that belongs to the current cluster (having the same `start` value).
        while self.idx < self.data.len() && self.data[self.idx].range.start == cluster_start {
            self.idx += 1;
        }

        Some((start..self.idx, cluster_start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_produces_clusters() {
        let glyph_ranges = vec![0..1, 2..3, 2..4, 2..5, 3..6, 4..7, 4..8, 5..9, 5..10];
        let expected_clusters = vec![(0..1, 0), (1..4, 2), (4..5, 3), (5..7, 4), (7..9, 5)];

        let glyphs = &mut glyph_ranges
            .iter()
            .map(|r| Glyph {
                range: r.clone(),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        let mut cluster_iterator = GlyphClusters::new(&glyphs);
        let mut actual_clusters = Vec::new();

        while let Some(cluster) = cluster_iterator.next() {
            actual_clusters.push(cluster);
        }

        assert_eq!(actual_clusters, expected_clusters);
    }
}
