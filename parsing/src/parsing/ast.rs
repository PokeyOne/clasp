#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub root: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Series(Vec<Expression>),
    Ident(String),
    Literal(Literal)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    StringLiteral(String)
}

impl Expression {
    pub fn type_name(&self) -> String {
        match self {
            Expression::Series(s) => {
                let subtypes = s.iter()
                    .map(|e| e.type_name())
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("Series<{}>", subtypes)
            },
            Expression::Ident(_) => "Ident".to_string(),
            Expression::Literal(lit) => {
                match lit {
                    Literal::StringLiteral(_) => "String",
                }.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_name() {
        let series = Expression::Series(vec![
            Expression::Ident("a".to_string()),
            Expression::Ident("b".to_string()),
            Expression::Literal(Literal::StringLiteral("c".to_string()))
        ]);

        assert_eq!(series.type_name(), "Series<Ident, Ident, String>");
    }
}