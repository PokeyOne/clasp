pub mod factory;

/// This is the format of the final output of the compiler. Currently the only
/// two options are the full binary program, or just the assembly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Assembly,
    Binary
}

/// These are the options that can be passed to the compiler. Usually these are
/// set via command line arguments, but they can also be passed directly to the
/// method that compiles the program.
pub struct RunOptions {
    output_path: String,
    input_path: String,
    output_format: OutputFormat
}

impl RunOptions {
    /// Creates a new RunOptions object with the given parameters.
    pub fn new(output_path: String, input_path: String, output_format: OutputFormat) -> RunOptions {
        RunOptions {
            output_path: output_path,
            input_path: input_path,
            output_format: output_format
        }
    }

    /// Returns the path to the output file.
    pub fn output_path(&self) -> &String {
        &self.output_path
    }

    /// Returns the path to the input file.
    pub fn input_path(&self) -> &String {
        &self.input_path
    }

    /// Returns the output format.
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}
