fn main() {
    let input = read_input().unwrap();
    println!("{input}");
}

struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathedIoError")
            .field("path", &self.path)
            .field("inner", &self.inner)
            .finish()
    }
}

fn read_input() -> Result<String, PathedIoError> {
    let path = "src/foobar.txt";
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(PathedIoError { 
            path: path.into(),
            inner: e,
        })
    }
}
