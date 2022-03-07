use crate::parser::Parser;
use syntax::SyntaxKind::*;

/// PROGRAM
pub(super) fn root(p: &mut Parser) {
    let m = p.start();
    program(p);
    m.finish(p, ROOT);
}

/// STATEMENT(s)
fn program(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        match p.current() {
            IDENT | OPCODE => statement(p),
            _ => {
                let m = p.start();
                p.error("Expected statement");
                p.bump_any();
                m.finish(p, ERROR);
            }
        }
    }
    m.finish(p, PROGRAM);
}

/// LABEL? INSTR
fn statement(p: &mut Parser) {
    assert!(p.at(OPCODE) | p.at(IDENT));
    let m = p.start();
    match p.current() {
        OPCODE => instr(p),
        IDENT => {
            label(p);
            // look-ahead to combine with statement
            if p.at(OPCODE) {
                instr(p);
            }
        }
        _ => unreachable!(),
    }
    m.finish(p, STATEMENT);
}

/// OP ARG_LIST?
fn instr(p: &mut Parser) {
    assert!(p.at(OPCODE));
    let m = p.start();
    let has_args = op(p);
    if has_args {
        arg_list(p);
    }
    m.finish(p, INSTR);
}

/// OPCODE COND? HAS_ARGS?
fn op(p: &mut Parser) -> bool {
    assert!(p.at(OPCODE));
    let m = p.start();
    p.bump(OPCODE);
    p.eat(COND);
    let args = p.eat(HAS_ARGS);
    m.finish(p, OP);
    args
}

/// ARG, ARG ...
fn arg_list(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        arg(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    m.finish(p, ARG_LIST);
}

/// REG | SHIFT | LABEL | HASH | ADDRESS | REG_LIST
fn arg(p: &mut Parser) {
    let m = p.start();
    let c = p.current();
    let s = p.nth_str(0);
    match (c, s) {
        (k, _) if k.is_register() => register(p),
        (OPCODE, Some("ASR" | "LSL" | "LSR" | "ROR" | "RRX")) => shift(p),
        (IDENT, _) => label(p),
        (HASH, _) => immediate(p),
        (OPEN_SQUARE, _) => address(p),
        (OPEN_CURLY, _) => reg_list(p),
        _ => {
            p.error("expected an argument");
            m.finish(p, ERROR);
            return;
        }
    }
    m.finish(p, ARG);
}

/// { REG, REG ... }
fn reg_list(p: &mut Parser) {
    assert!(p.at(OPEN_CURLY));
    let m = p.start();
    p.bump(OPEN_CURLY);
    while !p.at(EOF) {
        register(p);
        if p.at(CLOSE_CURLY) {
            break;
        }
        p.expect(COMMA);
    }
    p.expect(CLOSE_CURLY);
    m.finish(p, REG_LIST);
}

/// offset: [REGISTER(, OFFSET)?]
/// pre   : [REGISTER, OFFSET]!
/// post  : [REGISTER], OFFSET
fn address(p: &mut Parser) {
    assert!(p.at(OPEN_SQUARE));
    let m = p.start();
    p.bump(OPEN_SQUARE);
    register(p);
    let kind = if p.eat(COMMA) {
        offset(p);
        p.expect(CLOSE_SQUARE);
        if p.eat(BANG) {
            ADDR_PRE
        } else {
            ADDR_OFF
        }
    } else {
        p.expect(CLOSE_SQUARE);
        if p.eat(COMMA) {
            offset(p);
            ADDR_POST
        } else {
            ADDR_OFF
        }
    };
    m.finish(p, kind);
}

/// IMMEDIATE
/// SIGN REGISTER (, SHIFT)?
fn offset(p: &mut Parser) {
    let m = p.start();
    let kind = if p.at(HASH) {
        immediate(p);
        OFFSET_IMM
    } else {
        sign(p);
        register(p);
        if p.eat(COMMA) {
            shift(p);
        }
        OFFSET_REG
    };
    m.finish(p, kind);
}

/// OP (REGISTER | IMMEDIATE)?
fn shift(p: &mut Parser) {
    assert!(p.at(OPCODE));
    let m = p.start();
    let code = p.nth_str(0).unwrap();
    op(p);
    match code {
        "RRX" => (),
        "ASR" | "LSL" | "LSR" | "ROR" => match p.current() {
            HASH => immediate(p),
            curr if curr.is_register() => register(p),
            _ => {
                let m = p.start();
                p.error("expected a register or an immediate");
                p.bump_any();
                m.finish(p, ERROR);
            }
        },
        _ => {
            p.error("attempted to shift using a non-shift instruction");
            m.finish(p, ERROR);
            return;
        }
    }
    m.finish(p, SHIFT);
}

/// \# SIGN LITERAL
fn immediate(p: &mut Parser) {
    assert!(p.at(HASH));
    let m = p.start();
    p.bump(HASH);
    sign(p);
    p.expect(LITERAL);
    m.finish(p, IMMEDIATE);
}

/// RN | SP | LR | PC !?
fn register(p: &mut Parser) {
    let m = p.start();
    if p.current().is_register() {
        p.bump_any();
        p.eat(BANG);
        m.finish(p, REGISTER);
    } else {
        p.bump_any();
        p.error("expected a register");
        m.finish(p, ERROR);
    };
}

/// IDENT :?
fn label(p: &mut Parser) {
    assert!(p.at(IDENT));
    let m = p.start();
    name(p);
    p.eat(COLON);
    m.finish(p, LABEL);
}

/// IDENT
fn name(p: &mut Parser) {
    let m = p.start();
    if p.at(IDENT) {
        p.bump(IDENT);
        m.finish(p, NAME);
    } else {
        p.error("expected a name");
        p.bump_any();
        m.finish(p, ERROR);
    }
}

/// (+|-)?
fn sign(p: &mut Parser) {
    let m = p.start();
    if !p.eat(PLUS) {
        p.eat(MINUS);
    }
    m.finish(p, SIGN);
}
