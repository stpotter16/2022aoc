#[repr(transparent)]
struct Item(u8);

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

fn main()-> color_eyre::Result<()> {
    let _a = Item::try_from(b'a')?;
    let _exclaim = Item::try_from(b'!')?;

    Ok(())
}
