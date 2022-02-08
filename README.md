# gtf-read-rs

Reads a gtf file and extracts gene-specific information. This is a library which exports one routine as of now:

```rust
pub fn get_gene_transcripts(
    file_name: &str,
    gene_name: &str,
) -> Result<HashMap<String, Vec<Exon>>>
```

## Examples

### Transcripts to Python dictionary

The transcripts_to_python.rs example illustrates one way to use this library. It prints out a Python dictionary from the Rust HashMap. This is how to run it using Cargo:

```
cargo run --example transcripts_to_python <gtf-file-name> <gene-name>
```
