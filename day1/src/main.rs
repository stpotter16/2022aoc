use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = read_input()?;
    println!("{input}");

    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let path = "src/foobar.txt";
    let input = std::fs::read_to_string(path).wrap_err("reading src/input.txt")?;
    Ok(input)
}
