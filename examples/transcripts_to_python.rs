use gtf_read_rs::get_gene_transcripts;
use std::env;
use std::time::Instant;

fn main() {
    if let Some(file_name) = env::args().nth(1) {
        if let Some(gene_name) = env::args().nth(2) {
            let instant = Instant::now();
            if let Ok(transcripts) = get_gene_transcripts(&file_name, &gene_name) {
                println!("transcripts = {{");
                for (transcript_id, sequence) in transcripts.iter() {
                    println!(" {}: [", transcript_id);
                    for exon in sequence {
                        println!("  ({},{}),", exon.start_offset, exon.end_offset);
                    }
                    println!(" ],");
                }
                println!("}}");
            }
            println!("# Completed in {} seconds", instant.elapsed().as_secs());
        }
    }
}
