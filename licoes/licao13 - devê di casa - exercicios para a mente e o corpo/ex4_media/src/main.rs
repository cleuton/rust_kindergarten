use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let arquivo = "notas.txt"; 
    let dados = fs::read_to_string(arquivo)?;
    let notas = dados.split(",");
    let mut soma = 0.0;
    let mut contador = 0;
    for nota in notas {
        let nota = nota.trim().parse::<f32>().unwrap();
        soma += nota;
        contador += 1;
    }
    let media = soma / contador as f32;
    println!("Média: {}", media);
    Ok(())
}