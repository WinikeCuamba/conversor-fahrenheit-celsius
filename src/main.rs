use std::io;

fn main() {
    
    
    let mut fahrenheit = String::new();
    print!("Digite o valor em fahrenheit que desejes converter: ");
    io::stdin().read_line(&mut fahrenheit)
        .expect("Falha ao tentar ler input do usuario");

    let fahrenheit: i32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    let celsius = ((fahrenheit - 32) / 9) * 5;

    println!("O valor de {} fahrenheit e igual a {} celsius",fahrenheit, celsius);


}
