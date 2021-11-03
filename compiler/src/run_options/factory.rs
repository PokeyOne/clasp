use super::{OutputFormat, RunOptions};

pub struct RunOptionsFactory {
  output_path: Option<String>,
  input_path: Option<String>,
  output_format: Option<OutputFormat>
}

impl RunOptionsFactory {
  pub fn new() -> RunOptionsFactory {
    RunOptionsFactory {
      output_path: None,
      input_path: None,
      output_format: None
    }
  }

  pub fn set_output_path(&mut self, output_path: String) {
    self.output_path = Some(output_path);
  }

  pub fn set_input_path(&mut self, input_path: String) {
    self.input_path = Some(input_path);
  }

  pub fn set_output_format(&mut self, output_format: OutputFormat) {
    self.output_format = Some(output_format);
  }

  pub fn construct(&self) -> Result<RunOptions, String> {
    Ok(RunOptions {
      output_path: match &self.output_path {
        Some(output_path) => output_path.clone(),
        None => "./a.out".to_string()
      },
      input_path: match &self.input_path {
        Some(input_path) => input_path.clone(),
        None => return Err("No input path specified".to_string())
      },
      output_format: match &self.output_format {
        Some(output_format) => output_format.clone(),
        None => OutputFormat::Binary
      }
    })
  }
}