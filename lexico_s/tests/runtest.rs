#[cfg(test)]
mod tests {
    use lexico_s::grammar::ProgramaParser; // Importación desde el crate `lexico_s`

    #[test]
    fn test_programa() {
        let parser = ProgramaParser::new();
        let codigo_fuente = "programa prueba vars x, y : entero; resultado : flotante; inicio { escribe (x); } fin";

        match parser.parse(codigo_fuente) {
            Ok(_) => println!("¡El programa se ha parseado correctamente!"),
            Err(e) => panic!("Error al parsear el programa: {:?}", e),
        }
    }

    #[test]
    fn test_declaraciones() {
        let parser = ProgramaParser::new();
        let codigo_fuente = "programa prueba vars x, y : entero; z : flotante; inicio { } fin";

        assert!(parser.parse(codigo_fuente).is_ok(), "Error al parsear declaraciones de variables");
    }

    #[test]
    fn test_expresiones_aritmeticas() {
        let parser = ProgramaParser::new();
        let codigo_fuente = "programa prueba inicio { x := 5 + 3 * (2 - 1); } fin";

        assert!(parser.parse(codigo_fuente).is_ok(), "Error al parsear expresiones aritméticas");
    }

    #[test]
    fn test_condiciones() {
        let parser = ProgramaParser::new();
        let codigo_fuente = "programa prueba inicio { si (x > 0) { escribe (x); } sino { escribe (0); } } fin";

        assert!(parser.parse(codigo_fuente).is_ok(), "Error al parsear condiciones");
    }

    #[test]
    fn test_ciclos() {
        let parser = ProgramaParser::new();
        let codigo_fuente = "programa prueba inicio { mientras (x < 10) { x := x + 1; } } fin";

        assert!(parser.parse(codigo_fuente).is_ok(), "Error al parsear ciclos");
    }

    #[test]
    fn test_errores_sintaxis() {
        let parser = ProgramaParser::new();
        let codigo_fuente = "programa prueba inicio { x := ; } fin"; // Error: expresión incompleta

        assert!(parser.parse(codigo_fuente).is_err(), "El parser no detectó un error de sintaxis");
    }
}