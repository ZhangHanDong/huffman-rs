# Usage:

```rust

extern crate huffman_rs;

fn main() {

    let mut encoder = HuffmanEncoder::new();
    let mut ascii_bytes: Vec<u8> = vec![];
    for i in 0..256 {
        ascii_bytes.push(i as u8);
    }

    let encoded_bytes: Vec<u8> = encoder.encode(ascii_bytes);
    println!("{:?}", encoded_bytes);
    let mut decoder = HuffmanDecoder::new();
    let decoded_bytes = decoder.decode(encoded_bytes).unwrap();

    println!("{:?}", decoded_bytes);

}

```
