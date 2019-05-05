use st::io;

fn main(){
    println!("Adivina el numero");

    println!("Por favor introduce tu corazonada");

    let mut corazonada=String:: new();

    io::stdin().read_line(&mut corazonada);
    .ok()
    .expect("Fallo al leer linea");
    println!("Tu corazonada fue: {}",corazonada);
}
