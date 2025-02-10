use nom::{types::CompleteStr, digit};

use super::opcode::Token;

named!(
    pub register <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register { reg_num: reg_num.parse().unwrap() }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }
}