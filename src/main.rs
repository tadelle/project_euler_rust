mod problems;
mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;

use std::time::Instant;
use std::io;
use problems::Problem;

fn get_result(number: i32) -> i64 {

    match number {
        1 => problem001::Problema::new().get_result(),
        2 => problem002::Problema::new().get_result(),
        3 => problem003::Problema::new().get_result(),
        4 => problem004::Problema::new().get_result(),
        5 => problem005::Problema::new().get_result(),
        6 => problem006::Problema::new().get_result(),
        7 => problem007::Problema::new().get_result(),
        _ => 0
    }
}

fn get_title(number: i32) -> String {

    match number {
        1 => problem001::Problema::new().get_title(),
        2 => problem002::Problema::new().get_title(),
        3 => problem003::Problema::new().get_title(),
        4 => problem004::Problema::new().get_title(),
        5 => problem005::Problema::new().get_title(),
        6 => problem006::Problema::new().get_title(),
        7 => problem007::Problema::new().get_title(),
        _ => String::from("Not implemented yet!")
    }
}

fn main() {
    loop {

        println!("Digite o número do problema ou x para sair: ");
        let mut problem_number = String::new();
        io::stdin()
            .read_line(&mut problem_number)
            .expect("Valor Inválido!");
        
        if problem_number.contains("x") || problem_number.contains("X"){
            break;
        }

        let number: i32 = problem_number.trim().parse::<i32>().unwrap();
        if number == 0{
            println!("Valor inválido!");
            continue;
        }
        let title = get_title(number);

        let now = Instant::now();
        let resultado: i64 = get_result(number);
        let tempo = now.elapsed();
        println!("Problem: {number} - {title}");
        println!(
            "Resultado: {resultado}. tempo: {} ms ({} ticks) ({} ns)",
            tempo.as_millis(),
            tempo.as_nanos() / 100,
            tempo.as_nanos()
        );
    }
}
