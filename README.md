# textractor
A simple text extractor for PDF files. This works as a microservice and can be used to extract text from PDF files very quickly for further processing. It is intended to be used to support other services that require text extraction from PDF files, namely machine learning models.

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