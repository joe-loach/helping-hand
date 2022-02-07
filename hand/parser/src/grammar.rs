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
            IDENT => statement(p),
            k if k.is_opcode() => statement(p),
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
    let m = p.start();
    match p.current() {
        IDENT => {
            label(p);
            // look-ahead to collapse line
            if p.current().is_opcode() {
                instr(p);
            }
        }
        k if k.is_opcode() => instr(p),
        _ => unreachable!(),
    }
    m.finish(p, STATEMENT);
}

/// CODE ARG_LIST
fn instr(p: &mut Parser) {
    let m = p.start();
    opcode(p);
    arg_list(p);
    m.finish(p, INSTR);
}

fn opcode(p: &mut Parser) {
    debug_assert!(p.current().is_opcode());
    let m = p.start();
    p.bump_any();
    m.finish(p, OPCODE);
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
    match p.current() {
        k if k.is_register() => register(p),
        k if k.is_shift() => shift(p),
        IDENT => label(p),
        HASH => immediate(p),
        OPEN_SQUARE => address(p),
        OPEN_CURLY => reg_list(p),
        _ => {
            m.abandon(p);
            return;
        },
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

/// [REGISTER]
/// [REGISTER], IMMEDIATE
/// [REGISTER,IMMEDIATE]
/// [REGISTER,IMMEDIATE]!
fn address(p: &mut Parser) {
    assert!(p.at(OPEN_SQUARE));
    let m = p.start();
    p.bump(OPEN_SQUARE);

    register(p);

    if p.eat(COMMA) {
        immediate(p);
        p.expect(CLOSE_SQUARE);
        p.eat(BANG);
    } else {
        p.expect(CLOSE_SQUARE);
        if p.eat(COMMA) {
            immediate(p);
        }
    }
    m.finish(p, ADDRESS);
}

fn shift(p: &mut Parser) {
    debug_assert!(p.current().is_shift());
    let m = p.start();
    let shift = p.current();
    p.bump(shift);
    match shift {
        RRX => (),
        _ => match p.current() {
            curr if curr.is_register() => register(p),
            HASH => immediate(p),
            _ => {
                let m = p.start();
                p.error("expected a register or an immediate");
                p.bump_any();
                m.finish(p, ERROR);
            }
        },
    }
    m.finish(p, SHIFT);
}

/// # (+|-)? LITERAL
fn immediate(p: &mut Parser) {
    assert!(p.at(HASH));
    let m = p.start();
    p.bump(HASH);
    if !p.eat(PLUS) {
        p.eat(MINUS);
    }
    p.expect(LITERAL);
    m.finish(p, IMMEDIATE);
}

/// RN | SP | LR | PC
fn register(p: &mut Parser) {
    let m = p.start();
    let kind = if p.current().is_register() {
        REGISTER
    } else {
        p.error("expected a register");
        ERROR
    };
    p.bump_any();
    m.finish(p, kind);
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
