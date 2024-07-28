use rayon::iter::plumbing::*;
use rayon::prelude::*;
use std::sync::Arc;
use tracing::info;

struct PairChunks<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T> PairChunks<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        PairChunks { slice }
    }
}

impl<'a, T: Sync + 'a> ParallelIterator for PairChunks<'a, T> {
    type Item = (&'a T, &'a T);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }
}

impl<'a, T: Sync + 'a> IndexedParallelIterator for PairChunks<'a, T> {
    fn len(&self) -> usize {
        self.slice.len() / 2
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(self)
    }
}

impl<'a, T: Sync + 'a> Producer for PairChunks<'a, T> {
    type Item = (&'a T, &'a T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .chunks(2)
            .filter_map(|chunk| {
                if let [a, b] = chunk {
                    Some((a, b))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let mid = index * 2;
        (
            PairChunks::new(&self.slice[..mid]),
            PairChunks::new(&self.slice[mid..]),
        )
    }
}

pub async fn iterate_in_chunks() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<(&i32, &i32)> = PairChunks::new(&data).collect();

    // info!("{:?}", result);
    for (a, b) in result {
        info!("{} {}", a, b);
    }
}
