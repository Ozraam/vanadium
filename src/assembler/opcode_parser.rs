use nom::types::CompleteStr;
use nom::alpha1;
use super::opcode::Token;
use crate::instruction::Opcode;

named!(pub opcode<CompleteStr, Token>,
  do_parse!(
      opcode: alpha1 >>
      (
        Token::Op{code: Opcode::from(opcode)}
      )
  )
);

mod tests {
    #![allow(unused_imports)]
    use super::opcode;
    use crate::instruction::Opcode;
    use crate::assembler::opcode::Token;
    use nom::types::CompleteStr;

    #[test]
    fn test_opcode() {
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));
        let result = opcode(CompleteStr("aold"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}