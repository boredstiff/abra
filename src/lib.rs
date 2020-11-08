mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);
        (s, Self(number.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, operation) = utils::extract_operator(s);

        let operation = match operation {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => panic!("Bad operator"),
        };

        (s, operation)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub left: Number,
    pub right: Number,
    pub operation: Operation,
}

impl Expression {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, left) = Number::new(s);
        let (s, _) = utils::extract_whitespace(s);
        let (s, operation) = Operation::new(s);
        let (s, _) = utils::extract_whitespace(s);
        let (s, right) = Number::new(s);
        (s, Self { left, right, operation })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Operation::new("+"), ("", Operation::Add));
    }

    #[test]
    fn parse_subtract_op() {
        assert_eq!(Operation::new("-"), ("", Operation::Subtract));
    }

    #[test]
    fn parse_multiply_op() {
        assert_eq!(Operation::new("*"), ("", Operation::Multiply));
    }

    #[test]
    fn parse_divide_op() {
        assert_eq!(Operation::new("/"), ("", Operation::Divide));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1+2"),
            (
                "",
                Expression {
                    left: Number(1),
                    right: Number(2),
                    operation: Operation::Add,
                },
            ),
        );
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expression::new("2 * 2"),
            (
                "",
                Expression {
                    left: Number(2),
                    right: Number(2),
                    operation: Operation::Multiply,
                }
            )
        )
    }
}
