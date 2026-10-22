//! Exact, iterative set partitions. Compatibility is checked on complete heads,
//! never inferred by transitive closure. No pruning of physical alternatives.
use crate::{Candidate, Pitch, SearchError};
use std::collections::BTreeMap;

pub(crate) struct Partitions {
    buckets: Vec<Vec<usize>>,
    labels: Vec<Vec<usize>>,
}
impl Partitions {
    pub(crate) fn new(candidates: &[Candidate]) -> Self {
        let mut buckets = BTreeMap::<(u32, Pitch), Vec<usize>>::new();
        for (i, c) in candidates.iter().enumerate() {
            if c.grouping != 1 {
                buckets.entry((c.grouping, c.input_pitch)).or_default().push(i);
            }
        }
        let buckets: Vec<_> = buckets.into_values().collect();
        let labels = buckets.iter().map(|b| vec![0; b.len()]).collect();
        Self { buckets, labels }
    }

    pub(crate) fn count(&self, limit: u64) -> Result<u64, SearchError> {
        let mut count = 1u64;
        for bucket in &self.buckets {
            // Stirling numbers of the second kind, summed to obtain Bell(n).
            // Stop immediately above the operational bound, including overflow.
            let mut row = vec![1u64];
            for _ in 0..bucket.len() {
                let mut next = vec![0u64; row.len() + 1];
                for k in 1..next.len() {
                    next[k] = row[k - 1].checked_add(
                        row.get(k).copied().unwrap_or(0).checked_mul(k as u64)
                            .ok_or_else(too_many)?
                    ).ok_or_else(too_many)?;
                }
                let sum = next.iter().try_fold(0u64, |a, b| a.checked_add(*b)).ok_or_else(too_many)?;
                if sum > limit { return Err(too_many()); }
                row = next;
            }
            let bell: u64 = row.iter().sum();
            count = count.checked_mul(bell).ok_or_else(too_many)?;
            if count > limit { return Err(too_many()); }
        }
        Ok(count)
    }

    pub(crate) fn groups(&self) -> Vec<Vec<usize>> {
        let mut groups = Vec::new();
        for (bucket, labels) in self.buckets.iter().zip(&self.labels) {
            let start = groups.len();
            for (&index, &label) in bucket.iter().zip(labels) {
                if groups.len() == start + label { groups.push(Vec::new()); }
                groups[start + label].push(index);
            }
        }
        groups
    }

    pub(crate) fn advance(&mut self) -> bool {
        // Lexicographic restricted-growth strings: each set partition once,
        // shared first, separate last. Cartesian product across pickup keys.
        for labels in self.labels.iter_mut().rev() {
            for i in (1..labels.len()).rev() {
                let max = labels[..i].iter().copied().max().unwrap();
                if labels[i] <= max {
                    labels[i] += 1;
                    labels[i + 1..].fill(0);
                    return true;
                }
            }
            labels.fill(0);
        }
        false
    }
}
fn too_many() -> SearchError {
    SearchError::LimitExceeded("partizioni delle teste oltre il limite combinazioni".into())
}
