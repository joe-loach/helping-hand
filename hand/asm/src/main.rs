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
        for stmt in &ir {
            for (&atom, &data) in stmt.iter() {
                match atom {
                    Atom::Instruction => {
                        let op = middle::syn::<syntax::Opcode>(data);
                        print!("{}", op.as_str());
                    }
                    Atom::Condition => {
                        let cond = middle::syn::<syntax::Condition>(data);
                        print!(
                            "{} ",
                            if cond != syntax::Condition::AL {
                                cond.as_str()
                            } else {
                                ""
                            }
                        );
                    }
                    Atom::Shift => print!("{} ", data),
                    Atom::Register => {
                        let reg = middle::syn::<syntax::Register>(data);
                        print!("{} ", reg.as_str());
                    }
                    Atom::Label => print!("{}: ", data),
                    Atom::Value => print!("{} ", data),
                    Atom::Address => print!("@ "),
                    Atom::Offset => print!("+= "),
                    Atom::Sign => {
                        let sign = middle::syn::<syntax::Sign>(data);
                        print!(
                            "{}",
                            if sign == syntax::Sign::Negative {
                                sign.as_str()
                            } else {
                                ""
                            }
                        );
                    }
                    Atom::RegisterList => {
                        let list = middle::syn::<syntax::RegisterList>(data);
                        print!("{{{:016b}}}", list.flags);
                    }
                    Atom::Error => print!("ERROR "),
                }
            }
            println!();
        }
    }
    for err in middle::validate(&ir) {
        println!("error: {err}");
    }

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
