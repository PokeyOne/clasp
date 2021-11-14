#[cfg(test)]
mod tests;

use clasp_common::data_constants::WORD_SIZE;
use crate::text_processing::InstructionProcessResult;
use crate::text_processing::math_instructions::general_math;

pub fn cmp_process(words: Vec<String>) -> InstructionProcessResult {
    // TODO: Error instead of panic
    assert_eq!(3, words.len());

    let mut new_words = words.clone();
    // the general_math function requires a destination, so we will hijack that
    // method by tagging a destination and removing after.
    new_words.push("0x00".to_string());

    let mut result = match general_math(new_words) {
        Err(t) => return Err(t),
        Ok(v) => v
    };
    // remove the destination argument because a cmp instruction always goes
    // to the 'te' register
    result.0 = result.0[0..(result.0.len() - WORD_SIZE)].to_vec();

    Ok(result)
}
