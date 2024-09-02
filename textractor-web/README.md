# Textractor web API
This is a RESTful API for Textractor. It is build using [axum](https:://github.com/tokio-rs/axum) and the core [textractor]("../textractor-core") crate.

## Endpoints
- `GET /`: A simple health check endpoint.
- `POST /extract`: Extract text from various files.
- `GET /docs`: Get the status of the API.
- `GET /test`: OpenAPI documentation for the API.

## Running the API
To run the API, you need to have the rust toolchain installed. You can install it for your machine with [rustup](https://rustup.rs/).

### Running for development
After installing the rust toolchain, you can run the API with the following command:
```bash
cargo run
```

This will start the API at `localhost:8080`.

### Running for production
To run the API in production, you can build the API with the following command:
```bash
cargo run --release
```

### Building the binary
You can also optionally compile the binary with the following command:
```bash
cargo build --release
```

And then run the binary with:
```bash
./target/release/textractor
```

## Using the API
Here is a simple example of how to use the API with curl:
```bash
curl -X POST -F "file=@/path/to/file" http://localhost:8080/extract
```

This will extract the text from the file and return it as a JSON response. You can also use it from within python:

```python
import requests

url = "http://localhost:8080/extract"
files = {"file": open("/path/to/file", "rb")}
response = requests.post(url, files=files)

print(response.json())
```