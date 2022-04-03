const HELP: &str = "\
HAND Assembler

Usage: handasm [OPTIONS] INPUT

Options:
    -h, --help          Prints help information
    -o FILE             Write output to <file>
    --emit [asm|ir|ast] Type of output for the compiler to emit
";

use std::path::PathBuf;

#[derive(Debug)]
pub struct PrintHelp;

impl std::fmt::Display for PrintHelp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", HELP)
    }
}
impl std::error::Error for PrintHelp {}

pub fn parse() -> anyhow::Result<Cli> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        return Err(anyhow::anyhow!(PrintHelp));
    }

    let output = pargs.opt_value_from_os_str("-o", parse_path)?;
    let emit = pargs.opt_value_from_str("--emit")?.unwrap_or(Emit::Asm);
    let input = pargs.free_from_os_str(parse_path).map_err(|e| match e {
        pico_args::Error::MissingArgument => anyhow::anyhow!(PrintHelp),
        _ => anyhow::anyhow!(e),
    })?;

    let output = output.unwrap_or_else(|| input.with_extension(".o"));

    return Ok(Cli {
        input,
        output,
        emit,
    });

    fn parse_path(s: &std::ffi::OsStr) -> Result<PathBuf, &'static str> {
        Ok(s.into())
    }
}

pub struct Cli {
    /// Input file for the assembler
    pub input: PathBuf,
    /// Outut file for the assembler
    pub output: PathBuf,
    /// The type of output for the assembler to emit
    pub emit: Emit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Emit {
    Asm,
    IR,
    Ast,
}

impl core::fmt::Display for Emit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Emit::Asm => "asm",
            Emit::IR => "ir",
            Emit::Ast => "ast",
        };
        write!(f, "{text}")
    }
}

impl core::str::FromStr for Emit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Emit::*;
        let s = s.to_ascii_lowercase();
        let kind = match s.as_str() {
            "asm" => Asm,
            "ir" => IR,
            "ast" => Ast,
            _ => return Err(String::from("")),
        };
        Ok(kind)
    }
}
