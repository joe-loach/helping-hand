mod cli;

fn run() -> anyhow::Result<()> {
    let args = match cli::parse() {
        Ok(cli) => cli,
        Err(e) if e.is::<cli::PrintHelp>() => {
            println!("{e}");
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    };

    let src = std::fs::read_to_string(args.input)?;
    let emit = args.emit;

    let lexed = lexer::lex(&src);
    let parse = parser::parse(&lexed);

    if !parse.errors.is_empty() {
        for err in &parse.errors {
            eprintln!("error: {}", err);
        }
    }
    let root = ast::ast(parse);
    if emit == cli::Emit::Ast {
        println!("{:#?}", root);
        return Ok(());
    }
    for error in root.validate() {
        use ast::Node;

        match error.level {
            ast::Level::Error => eprintln!("error: {}", error.msg),
            ast::Level::Warn => eprintln!("warning: {}", error.msg),
        }
        let orig = error.element;
        let stmt = orig
            .ancestors()
            .find_map(ast::Statement::cast)
            .expect("all errors occur inside statements");
        let stmt_text = stmt.node().text().to_string();
        let stmt_text_sl = stmt_text.replace(['\n', '\r'], "");
        println!("{}", stmt_text_sl);
        let offset = orig.text_range().start() - stmt.node().text_range().start();
        let offset: usize = offset.try_into().unwrap();
        let offset = offset - (stmt_text.len() - stmt_text_sl.len());
        eprintln!(
            "{}{}",
            " ".repeat(offset),
            "^".repeat(orig.text_range().len().try_into().unwrap()),
        );
    }

    let ir = middle::lower(root);
    if emit == cli::Emit::IR {
        use middle::Atom;
        use syntax::{Condition, Directive, Opcode, Register, RegisterList, Sign};
        for stmt in &ir {
            for (&atom, &data) in stmt.iter() {
                match atom {
                    Atom::Directive => {
                        let dir = unsafe { middle::higher::<Directive>(data) };
                        print!("{} ", dir.as_str());
                    }
                    Atom::Instruction => {
                        let op = unsafe { middle::higher::<Opcode>(data) };
                        print!("{}", op.as_str());
                    }
                    Atom::Condition => {
                        let cond = unsafe { middle::higher::<Condition>(data) };
                        print!(
                            "{} ",
                            if cond != Condition::AL {
                                cond.as_str()
                            } else {
                                ""
                            }
                        );
                    }
                    Atom::Shift => print!("{} ", data),
                    Atom::Register => {
                        let reg = unsafe { middle::higher::<Register>(data) };
                        print!(
                            "{}{} ",
                            reg.as_str(),
                            if data & 0x10 != 0 { "!" } else { "" }
                        );
                    }
                    Atom::Label => print!("{}: ", data),
                    Atom::Number => print!("{} ", data),
                    Atom::Char => print!("{}", char::from_u32(data).unwrap()),
                    Atom::Bool => print!("{}", if data == 1 { "TRUE" } else { "FALSE" }),
                    Atom::Address => print!("@ "),
                    Atom::Offset => print!("+= "),
                    Atom::Sign => {
                        let sign = unsafe { middle::higher::<Sign>(data) };
                        print!(
                            "{}",
                            if sign == Sign::Negative {
                                sign.as_str()
                            } else {
                                ""
                            }
                        );
                    }
                    Atom::RegisterList => {
                        let list = unsafe { middle::higher::<RegisterList>(data) };
                        print!("{{{:016b}}}", list.flags);
                    }
                    Atom::Error => print!("ERROR "),
                }
            }
            println!();
        }
    }

    let binary = enc::encode(ir);

    std::fs::write(args.output, binary.as_bytes())?;

    Ok(())
}

const OK_CODE: i32 = 0;
const ERR_CODE: i32 = 1;

fn main() {
    let code = match run() {
        Ok(..) => OK_CODE,
        Err(e) => {
            println!("error: {}", e);
            ERR_CODE
        }
    };
    std::process::exit(code);
}
