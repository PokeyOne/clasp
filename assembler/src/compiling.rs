use clasp_common::data_constants::WORD_SIZE;
use clasp_common::data_types::{ByteCollection, Word};
use clasp_common::io::CCLASP_SIGNATURE;

use crate::label::LabelCollection;
use crate::text_processing;

#[cfg(test)]
mod tests;

// TODO: This should return a result. Errors should be passed up, not panic
/// This compiles raw clasm assembly text that you would see in a file into
/// the raw binary that should be handed to the runtime environment.
pub fn compile_text(input: String) -> Vec<u8> {
    let mut resulting_byte_code: Vec<u8> = Vec::new();

    // Append the clasp file signature to the data buffer
    for sig_byte in CCLASP_SIGNATURE {
        resulting_byte_code.push(sig_byte);
    }

    let mut labels = LabelCollection::new();
    let mut future_label_references: Vec<(String, u64)> = Vec::new();

    let mut line_index = 0;
    for line in input.lines() {
        line_index += 1;

        // TODO: Possible run this label check on trimmed line.
        if line.chars().nth(0) == Some(':') {
            labels.insert(line.to_string(), (resulting_byte_code.len()) as u64);
            continue;
        }

        let mut important_words: Vec<String> = Vec::new();

        for word in line.split(' ') {
            if word == ";;" {
                break;
            }

            let trimmed = word.trim();

            if trimmed == "" {
                continue;
            }

            // If the value matches a previously defined label, replace
            // TODO: This would fail if ((double derefs)) are allowed, not sure
            let label_retrieval_name =
                if trimmed.chars().nth(0) == Some('(') && trimmed.chars().last() == Some(')') {
                    trimmed[1..(trimmed.len() - 1)].to_string()
                } else {
                    trimmed.to_string()
                };
            match labels.retrieve(label_retrieval_name) {
                None => {}
                Some(loc) => {
                    // Formats to 18 characters because 8 bytes plus 2 for '0x'
                    important_words.push(format!("({:#018X})", loc));
                    continue;
                }
            };

            important_words.push(trimmed.to_string());
        }

        if important_words.len() == 0 {
            continue;
        }

        let processor = text_processing::get_function_for_instruction_name(&important_words[0]);
        let mut byte_code: Vec<u8> = match processor {
            None => panic!("Invalid argument {}", important_words[0]),
            Some(processor) => match processor(important_words) {
                Err(reason) => panic!("line {}: {:?}", line_index, reason),
                Ok(value) => {
                    let (data, futures) = value;

                    for future in futures {
                        let mut fut_copy = future.clone();
                        fut_copy.1 += resulting_byte_code.len() as u64;
                        future_label_references.push(fut_copy);
                    }

                    data
                }
            }
        };

        resulting_byte_code.append(&mut byte_code);
    }

    println!("Collected {} labels", labels.size());
    labels.print_ordered_list();

    println!(
        "Collected the following future labels: {:?}",
        &future_label_references
    );
    for refer in future_label_references {
        match labels.retrieve(refer.0.clone()) {
            Some(val) => {
                let val_bytes = (val as Word).to_bytes();
                for i in 0..8 {
                    let loc = i + refer.1;
                    resulting_byte_code[loc as usize] = val_bytes[i as usize];
                }
            }
            None => panic!("Undefined label {:?}", refer.0)
        }
    }

    return resulting_byte_code;
}
