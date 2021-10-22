use std::env;

// TODO: Extend this documentation.
/// A processed command line argument
#[derive(Debug)]
pub struct CLArg {
    pub name: Option<String>,
    pub value: String
}

impl CLArg {
    pub fn named(name: String, value: String) -> CLArg {
        CLArg {
            name: Some(name),
            value: value
        }
    }

    pub fn anonymous(value: String) -> CLArg {
        CLArg {
            name: None,
            value: value
        }
    }

    pub fn is_named(&self) -> bool {
        match self.name {
            None => false,
            Some(_) => true
        }
    }

    pub fn is_anonymous(&self) -> bool {
        match self.name {
            None => true,
            Some(_) => false
        }
    }
}

/// This is used to specify certain named arguments that program may accept so
/// that the process_args method can appropriate group and process them. Named
/// args are arguments that usually start with two dashes and/or have set names.
/// For example, the runtime environment has the named arguments "--nodump" and
/// "--dump". These are examples of singleton named arguments.
///
/// The has_secondary flag would be true if the following argument after the
/// name should be included in the resulting CLArg. For example you may have an
/// option such as "--output-file <path>", in which case you would want to
/// include that path. However, for something like "--nodump" there will never
/// be anything that comes after it.
///
/// Aliases are other names that will be mapped to the same argument. This is
/// useful for cases where you may have "--version", but "-v" will do the same
/// thing.
#[derive(Debug)]
pub struct NamedArgSpec {
    name: String,
    has_secondary: bool,
    aliases: Vec<String>,
    all_names: Vec<String>
}

impl NamedArgSpec {
    /// Create a new NamedArgSpec object. The arguments name, has_secondary,
    /// and aliases are direct maps to the same name on the resulting struct.
    /// The one exception to this is that you may suply None for aliases and an
    /// empty Vec will be constructed for you.
    ///
    /// The all_names property is automatically constructed for you.
    pub fn new(name: &str, has_secondary: bool, aliases: Option<Vec<String>>) -> NamedArgSpec {
        let mut result = NamedArgSpec {
            name: name.to_string(),
            has_secondary: has_secondary,
            aliases: match aliases {
                Some(value) => value,
                None => Vec::new() as Vec<String>
            },
            all_names: Vec::new() as Vec<String>
        };

        result.all_names.push(result.name.clone());
        for ele in &result.aliases {
            result.all_names.push(ele.clone());
        }

        result
    }
}

/// Processes the command line arguments that are passed to the running program,
/// then returns them back in an easier to digest format. The first argument in
/// the returned vector will be the name of the program.
///
/// The return result will be a Vec with items of type CLArg.
pub fn process_args(arg_specs: Vec<NamedArgSpec>) -> Vec<CLArg> {
    /// The returnable CLArg objects
    let mut result: Vec<CLArg> = Vec::new();
    /// The environment arguments
    let args: Vec<String> = env::args().collect();

    /// Temp storage for name to be joined with the next argument
    let mut name: Option<String> = None;

    'arg_loop: for arg in args {
        match name {
            Some(n) => {
                result.push(CLArg::named(n, arg));
                name = None;
            }
            None => {
                for spec in &arg_specs {
                    // Check if the spec name or any aliases match the current argument
                    let all_names = spec.all_names.clone();
                    if all_names.into_iter().any(|val| val == arg) {
                        if spec.has_secondary {
                            name = Some(spec.name.clone());
                        } else {
                            result.push(CLArg::named(spec.name.clone(), arg.clone()));
                        }

                        continue 'arg_loop;
                    }
                }

                result.push(CLArg::anonymous(arg));
            }
        };
    }

    return result;
}
