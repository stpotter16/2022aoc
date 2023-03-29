mod item {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!(
                        "{} is not a valid item",
                        value as char
                )),
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }

    impl Item {
        pub(crate) fn score(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 -b'A') as usize,
                _ => unreachable!(),
            }
        }
    }
}

use crate::item::Item;
use itertools::Itertools;

fn main()-> color_eyre::Result<()> {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| b.try_into().unwrap())
                .fold([0u8; 53], |mut acc, x: Item| {
                    acc[x.score()] = 1;
                    acc
                })
        })
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            chunks
                .reduce(|mut a, b| {
                    for (a, b) in a.iter_mut().zip(b.iter()) {
                        *a += *b
                    }
                    a
                })
                .expect("we always have 3 chunks")
                .iter()
                .position(|&b| b == 3)
                .expect("There should be one item in common")
        })
        .sum();

    dbg!(sum);
    Ok(())
}
