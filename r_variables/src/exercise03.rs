pub fn variable_scope() {
    // Escopo de variáveis
    // 1. Declare uma variável inteira chamada `x` dentro de uma função e tente acessá-la fora do escopo dessa função.
    // 2. Declare uma variável inteira `y` dentro de um bloco de código delimitado por chaves `{}` e tente acessá-la fora desse bloco.
    // Coloque suas declarações de variáveis aqui:

    // {
    //     let x = 10;
    // }

    random_function();

    // to compile
    let x = 1;
    let y = 2;

    // Tente acessar as variáveis fora de seus respectivos escopos
    println!("Value of x: {}", x); // Esperado: Erro de compilação
    println!("Value of y: {}", y); // Esperado: Erro de compilação
}

fn random_function() {
    // let y = 5;
}
