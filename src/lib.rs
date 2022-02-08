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

fn get_unquoted_string(possibly_quoted: String) -> String {
    possibly_quoted
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_string()
}

struct GtfLineAttributes {
    attribute_string: String,
    pos: usize,
}

impl GtfLineAttributes {
    fn new(attribute_string: String) -> GtfLineAttributes {
        GtfLineAttributes {
            attribute_string: attribute_string,
            pos: 0,
        }
    }
}

impl Iterator for GtfLineAttributes {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let kv = self.attribute_string.split(";").nth(self.pos)?;
        let mut keyvalue = kv.split_whitespace();
        let key = keyvalue.next()?;
        let value = keyvalue.next()?;
        self.pos += 1;
        Some((key.to_string(), value.to_string()))
    }
}

fn get_gtf_line_parts(line: &mut String) -> Option<GtfLineParts> {
    let mut parts = line.trim_end_matches("\n").split("\t");
    let mut gtf_line_parts = GtfLineParts {
        kind: String::from(""),
        start_offset: 0,
        end_offset: 0,
        gene_name: String::from(""),
        transcript_id: String::from(""),
    };

    gtf_line_parts.kind = parts.nth(2)?.to_string();

    gtf_line_parts.start_offset = parts.next()?.parse::<i32>().unwrap();

    gtf_line_parts.end_offset = parts.next()?.parse::<i32>().unwrap();

    let _next_two_fields = parts.nth(2)?;

    let mut attribute_string = String::new();
    for attribute in parts {
        attribute_string.push_str(attribute);
        attribute_string.push_str(" ");
    }

    for (key, value) in GtfLineAttributes::new(attribute_string) {
        if key == "gene_name" {
            gtf_line_parts.gene_name = get_unquoted_string(value);
        } else if key == "transcript_id" {
            gtf_line_parts.transcript_id = get_unquoted_string(value);
        }
    }

    Some(gtf_line_parts)
}

fn read_gtf_file(file: File, gene_name: &str, transcripts: &mut HashMap<String, Vec<Exon>>) {
    let mut reader = BufReader::<File>::new(file);
    let mut line: String = String::new();
    while let Ok(chars_read) = reader.read_line(&mut line) {
        if chars_read > 0 {
            if let Some(gtf_parts) = get_gtf_line_parts(&mut line) {
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
    let file = File::open(file_name)?;
    let mut transcripts = HashMap::<String, Vec<Exon>>::new();
    read_gtf_file(file, gene_name, &mut transcripts);
    Ok(transcripts)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
