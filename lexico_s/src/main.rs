// src/main.rs
pub mod ast;

// Este macro le dice a Rust que incluya el archivo generado por LALRPOP
// Se llamará igual que tu archivo .lalrpop (en este caso "grammar")
lalrpop_util::lalrpop_mod!(pub grammar);

fn main() {
    // LALRPOP expone un parser con el nombre de tu regla principal + "Parser"
    let parser = grammar::ExprParser::new();

    let codigo_fuente = "2 + 3 * 4";
    
    match parser.parse(codigo_fuente) {
        Ok(ast) => {
            println!("Parseo exitoso!");
            println!("{:#?}", ast);
        },
        Err(e) => {
            println!("Error de sintaxis: {:?}", e);
        }
    }
}