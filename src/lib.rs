use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

#[derive(Debug)]
pub struct Exon {
    pub start_offset: i32,
    pub end_offset: i32,
}

struct GtfLineParts {
    kind: String,
    start_offset: i32,
    end_offset: i32,
    gene_name: String,
    transcript_id: String,
}

fn get_gtf_line_parts(line: &mut String) -> GtfLineParts {
    let mut parts = line.split("\t");
    let mut gtf_line_parts = GtfLineParts {
        kind: String::from(""),
        start_offset: 0,
        end_offset: 0,
        gene_name: String::from(""),
        transcript_id: String::from(""),
    };

    if let Some(kind) = parts.nth(2) {
        gtf_line_parts.kind = kind.to_string();

        if let Some(start_offset) = parts.next() {
            gtf_line_parts.start_offset = start_offset.parse::<i32>().unwrap();

            if let Some(end_offset) = parts.next() {
                gtf_line_parts.end_offset = end_offset.parse::<i32>().unwrap();

                if let Some(_) = parts.nth(2) {
                    let mut attribute_string = String::new();
                    for attribute in parts {
                        attribute_string.push_str(attribute);
                        attribute_string.push_str("\t");
                    }

                    let kvpairs = attribute_string.split(";");
                    for kv in kvpairs {
                        let mut keyvalue = kv.split(" ");
                        let key = loop {
                            if let Some(key) = keyvalue.next() {
                                if key != "" {
                                    break key;
                                }
                            }
                        };
                        if key == "gene_name" {
                            gtf_line_parts.gene_name = keyvalue.next().unwrap().to_string();
                        } else if key == "transcript_id" {
                            gtf_line_parts.transcript_id = keyvalue.next().unwrap().to_string();
                        }
                    }
                }
            }
        }
    }

    gtf_line_parts
}

fn read_gtf_file(file: File, gene_name: &str, transcripts: &mut HashMap<String, Vec<Exon>>) {
    let mut reader = BufReader::<File>::new(file);
    let mut line: String = String::new();
    while let Ok(chars_read) = reader.read_line(&mut line) {
        if chars_read > 0 {
            let gtf_parts = get_gtf_line_parts(&mut line);
            if gtf_parts.kind == "exon" && gtf_parts.gene_name == gene_name {
                let exon = Exon {
                    start_offset: gtf_parts.start_offset,
                    end_offset: gtf_parts.end_offset,
                };
                if let Some(sequence) = transcripts.get_mut(&gtf_parts.transcript_id) {
                    sequence.push(exon);
                } else {
                    let mut sequence = Vec::<Exon>::new();
                    sequence.push(exon);
                    transcripts.insert(gtf_parts.transcript_id, sequence);
                }
            }
            line.clear();
        } else {
            break;
        }
    }
}

pub fn get_gene_transcripts(
    file_name: &str,
    gene_name: &str,
) -> Result<HashMap<String, Vec<Exon>>> {
    match File::open(file_name) {
        Ok(of) => {
            let mut transcripts = HashMap::<String, Vec<Exon>>::new();
            read_gtf_file(of, gene_name, &mut transcripts);
            Ok(transcripts)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
