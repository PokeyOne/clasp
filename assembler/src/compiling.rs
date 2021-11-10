use clasp_common::data_constants::WORD_SIZE;
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

    let mut line_index = 0;
    for line in input.lines() {
        line_index += 1;

        // TODO: Possible run this label check on trimmed line.
        if line.chars().nth(0) == Some(':') {
            labels.insert(
                line.to_string(),
                (resulting_byte_code.len() - WORD_SIZE) as u64
            );
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
            let label_retrieval_name =
                if trimmed.chars().nth(0) == Some('(') && trimmed.chars().last() == Some(')') {
                    trimmed[1..(trimmed.len() - 1)].to_string()
                } else {
                    trimmed.to_string()
                };
            match labels.retrieve(label_retrieval_name) {
                None => trimmed,
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

        let byte_code_result = match (&important_words[0]) as &str {
            "nop" => text_processing::nop_process(important_words),
            "mov" => text_processing::mov_process(important_words),
            "outr" => text_processing::outr_process(important_words),
            "end" => text_processing::end_process(important_words),
            "add" => text_processing::add_process(important_words),
            "sub" => text_processing::sub_process(important_words),
            "mul" => text_processing::mul_process(important_words),
            "div" => text_processing::div_process(important_words),
            "pow" => text_processing::pow_process(important_words),
            "jmp" => text_processing::jmp_process(important_words),
            _ => panic!(
                "Syntax error, unexpected instruction at line {}",
                line_index
            )
        };

        let mut byte_code = match byte_code_result {
            Ok(val) => val,
            Err(err) => panic!("line {}: {:?}", line_index, err)
        };

        resulting_byte_code.append(&mut byte_code);
    }

    println!("Collected {} labels", labels.size());
    labels.print_ordered_list();

    return resulting_byte_code;
}
