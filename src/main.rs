fn main() {
    funcao_print();

    let y: () = {  // y == ()
        funcao_print();
        let x = 5;
    };

    let x: i32 = { //x==35
        funcao_print();
        35 //nao pode ter ; para atribuir
    };

    println!("{:?}", x);

    println!("{:?}", add_numbers(10, 50));


    let input = "23 32 65 34 23 64";

    let result: Vec<i32> = input.split(' ').map(|c| c.parse::<i32>().unwrap()).map(|n| n * 2).collect();
    
    println!("{:?}", result);

    let z = 10;
    //let ç = z;
    let ç = &z; //ponteiro, normalmente ele copia o valor (STACK)

    println!("{}", &ç);

    let z = String::new();
    let ç = z; // referencia a string é alterada, não copiada (HEAP)

    //println!("{:?}", z); //a variavel z fica invalida
    println!("{:?}", ç);
}

fn add_numbers(x: i32, y: i32) -> i32{
    if x == 10 {
        return 15;
    }
    x + y //retorna
}

fn funcao_print(){
    println!("Hello, world!");
}