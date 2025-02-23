## About this project
This is an implementation of Huffman Coding in Rust. The idea of Huffman Coding is to minimize the size of data by using shorter codes for frequent symbols without losing any information.

Refer to: [Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding)

## Usage
To compress file:
```rs
cargo run -- --input [input_path] --output [output_path] compress
```
  
To decompress file:
```rs
cargo run -- --input [input_path] --output [output_path] decompress
```
