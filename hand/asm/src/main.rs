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
        if let Some(stmt) = orig.ancestors().find_map(ast::Statement::cast) {
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
            )
        } else {
            panic!("all errors occur inside statements");
        }
    }

    let ir = middle::lower(root);
    if emit == cli::Emit::IR {
        // for stmt in &ir {
        //     for atom in stmt.atoms() {
        //         print!("{atom:?} ");
        //     }
        //     println!();
        // }
    }
    for err in middle::validate(&ir) {
        println!("error: {err}");
    }

    Ok(())
}

const OK_CODE: i32 = 0;
const ERR_CODE: i32 = 1;

fn main() {
    match run() {
        Ok(..) => {
            std::process::exit(OK_CODE);
        }
        Err(e) => {
            println!("error: {}", e);
            std::process::exit(ERR_CODE);
        }
    }
}
