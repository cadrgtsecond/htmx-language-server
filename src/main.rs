use std::{fs::File, io::Read, path::Path};
use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    let language = tree_sitter_html::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Could not load html parser");
    let mut code_file = File::open(Path::new("test/contact-app/templates/archive_ui.html")).expect("Could not open html");
    let mut code = String::new();
    code_file.read_to_string(&mut code).expect("Could not read html");

    let tree = parser.parse(code, None).expect("Could not parse html");
    println!("{}", tree.root_node());
}