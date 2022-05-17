use std::collections::HashMap;

use crate::AtomKind::*;
use crate::IR;
use ast::Token;

type LabelMap = HashMap<String, u32>;

/// Outputs IR in the form:
///
/// LABEL
///     (
///     | INSTRUCTION CONDITION ARGS?
///     | DIRECTIVE ARGS?
///     )?
pub(super) fn ir(root: ast::Root, mut labels: LabelMap) -> IR {
    let mut ir = IR::new();

    for (stmt, i) in root.program().statements().zip(0_u32..) {
        // LABEL
        ir.push(Label, i);
        // BODY?
        if let Some(body) = stmt.body() {
            match body {
                ast::StmtBody::Instruction(instr) => instruction(&mut ir, instr, &labels),
                ast::StmtBody::Meta(meta) => directive(&mut ir, meta, &mut labels),
            }
        }
        // move onto the next statement
        ir.finish();
    }
    return ir;

    fn directive(ir: &mut IR, meta: ast::Meta, labels: &mut LabelMap) {
        let dir = meta.directive().syntax();
        ir.push(Directive, dir as u32);
        if let Some(args) = meta.args() {
            for a in args {
                arg(ir, a, labels);
            }
        }
    }

    fn instruction(ir: &mut IR, instr: ast::Instruction, labels: &LabelMap) {
        let op = instr.op();
        // INSTRUCTION
        ir.push(Instruction, op.code().syntax() as u32);
        // CONDITION
        ir.push(
            Condition,
            op.condition().map(|cond| cond.syntax()).unwrap_or_default() as u32,
        );
        // ARGS
        if let Some(args) = instr.args() {
            for a in args {
                arg(ir, a, labels);
            }
        }
    }

    fn arg(ir: &mut IR, arg: ast::Arg, labels: &LabelMap) {
        match arg.kind() {
            ast::ArgKind::Register(reg) => register(ir, reg),
            ast::ArgKind::Shift(sft) => shift(ir, sft),
            ast::ArgKind::Label(lbl) => {
                let name = lbl.name().ident();
                if let Some(&data) = labels.get(name.text()) {
                    ir.push(Label, data);
                } else {
                    ir.error("Label is not defined");
                }
            }
            ast::ArgKind::Immediate(imm) => immediate(ir, imm),
            ast::ArgKind::Literal(lit) => literal(ir, lit),
            ast::ArgKind::Address(addr) => {
                ir.push(Address, addr.syntax() as u32);
                register(ir, addr.base());
                if let Some(offset) = addr.offset() {
                    ir.push(Offset, offset.syntax() as u32);
                    match offset.kind() {
                        ast::OffsetKind::Immediate(o) => immediate(ir, o.immediate()),
                        ast::OffsetKind::Register(o) => {
                            sign(ir, o.sign());
                            register(ir, o.base());
                            if let Some(sft) = o.shift() {
                                shift(ir, sft);
                            }
                        }
                    }
                }
            }
            ast::ArgKind::RegList(r_list) => {
                let mut bits: u16 = 0;
                for reg in r_list.iter() {
                    let value = reg.syntax().value() as u16;
                    bits |= 1 << value;
                }
                ir.push(RegisterList, bits as u32)
            }
        }
    }

    fn sign(ir: &mut IR, sign: ast::Sign) {
        ir.push(Sign, sign.syntax() as u32)
    }

    /// 0..4: value
    /// 4: has_bang
    fn register(ir: &mut IR, reg: ast::Register) {
        let value = reg.syntax().value();
        let bang = reg.bang().is_some() as u32;
        ir.push(Register, value | bang << 4);
    }

    fn immediate(ir: &mut IR, imm: ast::Immediate) {
        sign(ir, imm.sign());
        literal(ir, imm.literal());
    }

    fn literal(ir: &mut IR, lit: ast::Literal) {
        match lit.kind() {
            ast::LiteralKind::Number(n) => match n.value() {
                Ok(value) => ir.push(Number, value),
                Err(e) => ir.error(format!("Number couldn't be parsed, {e}")),
            },
            ast::LiteralKind::String(s) => {
                let s = s.value();
                for c in s.chars() {
                    ir.push(Char, c as u32)
                }
            }
            ast::LiteralKind::Char(c) => {
                let c = c.value();
                ir.push(Char, c as u32)
            }
            ast::LiteralKind::Bool(b) => ir.push(Bool, b as u32),
        }
    }

    fn shift(ir: &mut IR, shift: ast::Shift) {
        ir.push(Shift, shift.syntax().value());
        if let Some(data) = shift.data() {
            match data {
                ast::ShiftData::Register(reg) => register(ir, reg),
                ast::ShiftData::Immediate(imm) => immediate(ir, imm),
            }
        }
    }
}

/// Must iterate through the ast collecting all the labels beforehand.
/// This allows forward references with label names.
pub(super) fn labels(root: &ast::Root) -> LabelMap {
    let mut map = HashMap::new();
    for (stmt, pos) in root.program().statements().zip(0_u32..) {
        if let Some(label) = stmt.label() {
            let name = label.name().ident().text().to_owned();
            map.insert(name, pos);
        }
    }
    map
}
