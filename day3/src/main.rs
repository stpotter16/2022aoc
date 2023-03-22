mod item {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
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

use item::Item;

fn main()-> color_eyre::Result<()> {
    let a = Item::try_from(b'a')?;
    a.score();
    a.score();

    Ok(())
}
