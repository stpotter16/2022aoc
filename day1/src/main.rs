use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::{Itertools, FoldWhile};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut group_sums = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            it.fold_while(None, |acc: Option<u64>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc),
            })
            .into_inner()
        })
        .map(Reverse);

    let mut heap = BinaryHeap::new();

    for init in (&mut group_sums).take(3) {
        heap.push(init);
    }

    for rest in group_sums {
        heap.push(rest);
        heap.pop();
    }

    let answer = heap.into_iter().map(|Reverse(v)| v).sum::<u64>();
    println!("{answer:?}");

    Ok(())
}

