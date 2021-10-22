use std::env;

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

#[derive(Debug)]
pub struct NamedArgSpec {
    name: String,
    has_secondary: bool,
    aliases: Vec<String>,
    all_names: Vec<String>
}

impl NamedArgSpec {
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
    let mut result: Vec<CLArg> = Vec::new();
    let args: Vec<String> = env::args().collect();

    let mut name: Option<String> = None;
    'arg_loop: for arg in args {
        match name {
            Some(n) => {
                result.push(CLArg::named(n, arg));
                name = None;
            },
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
