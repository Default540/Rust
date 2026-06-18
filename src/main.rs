const SECOUNDS_IN_MINUTE: u32 = 60;

fn main() {
    //comentario
    //cargo init | cargo run | cargo build | cargo fmt (formatação de texto) | cargo watch -x run
    println!("Ola mundo");

    // mut = mutavel
    let mut total: i32 = 40;

    println!("Total: {}", total);

    //não ocupa espaço na memoria, pode ser declarada fora de funções e não é possivel reinicializalas
    const a: i32 = 10;

    total = 50;

    println!("Total: {}", total);


    let numero = 10;

    println!("{}",numero);

    {
        // é permitido reinicializar variaveis
        let numero = "letra";
        println!("{}", numero);
    }

    println!("{}",numero);


    const SECOUNDS_IN_HOUR: u32 = SECOUNDS_IN_MINUTE * 60;
    println!("existe {} segundos em 1h", SECOUNDS_IN_HOUR)
}
