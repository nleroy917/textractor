<h1 align="center">textractor 🚜</h1>

A simple text extractor for various files. Includes core functionality for extracting text from files, a command-line interface, restful API, and python bindings. Project is a work in progress.

## How to use
There are four main ways to use `textractor`:
1. Command-line interface
2. Python bindings
3. Restful API
4. Core functionality

### Command-line interface
Install the CLI with `cargo`:
```bash
cargo install --git https://github.com/nleroy917/textractor
```
Then run the CLI with:
```bash
textractor <file>
```

### Python bindings
The python bindings are not yet available on PyPi, but you can install them from source. First, clone this repository:
```bash
git clone https://github.com/nleroy917/textractor
```
Then install the python bindings with:

```bash
cd textractor/textractor-py
make install
```

You need to ensure that you have the [`maturin`](https://github.com/PyO3/maturin) package installed. You can install it with:
```bash
pip install maturin
```

### Restful API
There is also a web server built with [`axum`](https://github.com/tokio-rs/axum) that can be run with:
```bash
cd textractor-web
cargo run --release
```

### Core functionality
Finally, you can use the core functionality in your own Rust project. Add the following to your `Cargo.toml`:
```toml
[dependencies]
textractor = { git = "https://github.com/nleroy917/textractor" }
```

Then you can use the library in your project with:
```rust
use std::

use textractor::extraction::extract;


fn main() {

    let path = std::path::Path::new("path/to/file");
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut data = Vec::new();

    reader.read_to_end(&mut data)?;

    let text = extract(&data)?;

    match text {
        Some(text) => Ok(text),
        None => Err(anyhow::anyhow!("Unsupported file type")),
    }

    println!("{}", text);
}
```

I am working to prioritize adding PPTX and XLSX support, as well as improving the text extraction for PDFs.

## Supported formats
- [X] Text (txt)
- [X] PDF
- [X] Word (docx)
- [ ] Excel (xlsx)
- [ ] PowerPoint (pptx)
- [ ] Images (png, jpg, etc)