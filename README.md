<h1 align="center">textractor 🚜</h1>

A simple text extractor for various files. Includes core functionality for extracting text from files, a command-line interface, restful API, and python bindings. PProject is a work in progress, and contributions are welcome.

## Running
There are two ways to run this service: using Docker or running natively with Rust.

### Docker

```bash
docker build -t textractor .
docker run -p 3000:3000 textractor
```

### Rust

```bash
cargo run --release
```

## Usage
To extract text from a PDF file, send a POST request to the `/extract` endpoint with the PDF file as a form-data parameter named `file`.

```bash
curl -X POST -F "file=@/path/to/file.pdf" http://localhost:3000/extract
```

## Supported formats
- [X] PDF
- [X] Word (docx)
- [ ] Excel (xlsx)
- [ ] PowerPoint (pptx)
- [ ] Images (png, jpg, etc)
- [ ] Text (txt)