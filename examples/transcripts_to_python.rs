use std::env;
use gtf_read_rs::read_gtf_file;

fn main() {
    if let Some(file_name) = env::args().nth(1) {
        if let Some(gene_name) = env::args().nth(2) {
            read_gtf_file (&file_name, &gene_name);
        }
    }
}
