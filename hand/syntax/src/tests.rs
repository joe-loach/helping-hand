use std::{
    collections::HashMap,
    env, fs, io,
    path::{Path, PathBuf},
};

use roxmltree::{Document, Node};

const ARM: &str = "../../arm";

#[test]
fn generate_instructions() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&manifest).join("src/generated.rs");

    let files = xml_paths(ARM)?;

    let mut instructions = HashMap::new();
    for path in files {
        let contents = fs::read_to_string(path).expect("file exists");
        let doc = Document::parse(&contents).expect("file contains valid xml");
        match encodings(&doc) {
            Some(enc) => {
                for e in enc {
                    let name = {
                        let mnemonic = e.mnemonic();
                        if let Some(i) = mnemonic.find('{') {
                            mnemonic.split_at(i).0
                        } else {
                            mnemonic
                        }
                        .to_string()
                    };
                    let asm = e.template();
                    let text = asm.text();
                    let v = instructions.entry(name).or_insert_with(Vec::new);
                    v.push(text);
                }
            }
            None => continue,
        };
    }
    let mut contents = String::new();
    let mut instructions: Vec<(&String, &Vec<String>)> = instructions.iter().collect();
    instructions.sort_by(|a, b| a.0.cmp(b.0));
    for (name, encodings) in instructions {
        let list = encodings.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&format!(r#""{}","#, x));
            acc
        });
        let line = format!(
            "pub(crate) struct {name};\nimpl {name} {{ const ENC: &'static [& 'static str] = &[{list}]; }}\n"
        );
        contents.push_str(&line);
    }
    fs::write(&dest_path, contents)?;

    Ok(())
}

struct Encoding<'a, 'b>(Node<'a, 'b>);
struct Template<'a, 'b>(Node<'a, 'b>);

impl<'a, 'b> Encoding<'a, 'b> {
    pub fn mnemonic(&self) -> &str {
        let node = self
            .vars()
            .children()
            .find(|node| node.attribute("key") == Some("mnemonic"))
            .unwrap();
        node.attribute("value").unwrap()
    }

    fn vars(&self) -> Node {
        self.0
            .children()
            .find(|node| node.has_tag_name("docvars"))
            .unwrap()
    }

    pub fn template(&self) -> Template {
        Template(
            self.0
                .children()
                .find(|node| node.has_tag_name("asmtemplate"))
                .unwrap(),
        )
    }
}

impl<'a, 'b> Template<'a, 'b> {
    pub fn text(&self) -> String {
        self.0.children().filter_map(|node| node.text()).fold(
            String::new(),
            |mut acc: String, text| {
                acc.push_str(text);
                acc
            },
        )
    }
}

fn encodings<'a, 'input>(
    doc: &'a Document<'input>,
) -> Option<impl Iterator<Item = Encoding<'a, 'input>>> {
    let instruction = doc.root_element();
    let classes = instruction
        .children()
        .find(|node| node.has_tag_name("classes"))?;
    let a1 = classes
        .children()
        .find(|node| node.has_tag_name("iclass") && node.attribute("name") == Some("A1"))?;

    Some(a1.children().filter_map(|node| {
        if node.has_tag_name("encoding") {
            Some(Encoding(node))
        } else {
            None
        }
    }))
}

fn xml_paths(dir: impl AsRef<Path>) -> io::Result<impl Iterator<Item = PathBuf>> {
    const XML_EXT: &str = "xml";
    Ok(fs::read_dir(dir)?
        .filter_map(|path| Some(path.ok()?.path()))
        .filter(|path| {
            path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(XML_EXT)
        }))
}
