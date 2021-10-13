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

pub fn process_args() -> Vec<CLArg> {
    let mut result: Vec<CLArg> = Vec::new();
    let args: Vec<String> = env::args().collect();

    let mut name: Option<String> = None;
    for arg in args {
        match name {
            Some(n) => {
                result.push(CLArg::named(n, arg));
                name = None;
            },
            None => {
                if arg.chars().nth(0).map_or(false, |a| a == '-') {
                    name = Some(arg);
                } else {
                    result.push(CLArg::anonymous(arg));
                }
            }
        };
    }

    return result;
}
