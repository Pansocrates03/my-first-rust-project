# Lenguaje Patito
Esta carpeta tiene el parser de léxico y gramática realizado en rust con la herramienta LALRPOP. El parser acepta el lenguaje patito.

## Ejemplo del lenguaje patito

```
programa prueba

vars x, y : entero;
resultado : flotante;

inicio {
    escribe (x);
}

fin
```

## Limtaciones del lenguaje patito
- No acepta comentarios


## 1. To run the grammar parser just do

```
cargo run
```

## 2. To run the tests of the grammar parser, do:

```
cargo test
```