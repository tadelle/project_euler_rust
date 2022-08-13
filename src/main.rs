mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;
mod problem010;
mod problem011;
mod problem012;
mod problem013;
mod problem014;
mod problem015;
mod problem016;
mod problem017;
mod problem018;
mod problem019;
mod problem020;
mod problem021;
mod problem028;
mod problem034;
mod problem039;
mod problem048;
mod problem051;
mod problem067;
mod problems;

use colored::*;
use num_cpus;
use problems::Problem;
use std::env::args;
use std::io;
use std::time::{Duration, Instant};
use sysinfo::SystemExt;

fn get_result(number: i32) -> i64 {
    match number {
        1 => problem001::Problema::new().get_result(),
        2 => problem002::Problema::new().get_result(),
        3 => problem003::Problema::new().get_result(),
        4 => problem004::Problema::new().get_result(),
        5 => problem005::Problema::new().get_result(),
        6 => problem006::Problema::new().get_result(),
        7 => problem007::Problema::new().get_result(),
        8 => problem008::Problema::new().get_result(),
        9 => problem009::Problema::new().get_result(),
        10 => problem010::Problema::new().get_result(),
        11 => problem011::Problema::new().get_result(),
        12 => problem012::Problema::new().get_result(),
        13 => problem013::Problema::new().get_result(),
        14 => problem014::Problema::new().get_result(),
        15 => problem015::Problema::new().get_result(),
        16 => problem016::Problema::new().get_result(),
        17 => problem017::Problema::new().get_result(),
        18 => problem018::Problema::new().get_result(),
        19 => problem019::Problema::new().get_result(),
        20 => problem020::Problema::new().get_result(),
        21 => problem021::Problema::new().get_result(),
        28 => problem028::Problema::new().get_result(),
        34 => problem034::Problema::new().get_result(),
        39 => problem039::Problema::new().get_result(),
        48 => problem048::Problema::new().get_result(),
        51 => problem051::Problema::new().get_result(),
        67 => problem067::Problema::new().get_result(),
        _ => 0,
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
        8 => problem008::Problema::new().get_title(),
        9 => problem009::Problema::new().get_title(),
        10 => problem010::Problema::new().get_title(),
        11 => problem011::Problema::new().get_title(),
        12 => problem012::Problema::new().get_title(),
        13 => problem013::Problema::new().get_title(),
        14 => problem014::Problema::new().get_title(),
        15 => problem015::Problema::new().get_title(),
        16 => problem016::Problema::new().get_title(),
        17 => problem017::Problema::new().get_title(),
        18 => problem018::Problema::new().get_title(),
        19 => problem019::Problema::new().get_title(),
        20 => problem020::Problema::new().get_title(),
        21 => problem021::Problema::new().get_title(),
        28 => problem028::Problema::new().get_title(),
        34 => problem034::Problema::new().get_title(),
        39 => problem039::Problema::new().get_title(),
        48 => problem048::Problema::new().get_title(),
        51 => problem051::Problema::new().get_title(),
        67 => problem067::Problema::new().get_title(),
        _ => String::from("Not implemented yet!"),
    }
}

fn print_color(num_problem: i32, title: String, answer: i64, time: Duration, info: bool) {
    if answer == 0 {
        println!("{}", format!("Problem {num_problem} - {title}").red());
        println!("{}\n", "Answer: ".red());
    } else {
        println!("{}", format!("Problem {num_problem} - {title}").green());
        println!("{}\n", format!("Answer: {}", answer).green());
    }
    if info {
        println!(
            "{}",
            format!(
                "Response time: {} ms ({} ticks) ({} ns)",
                time.as_millis(),
                time.as_nanos() / 100,
                time.as_nanos()
            )
            .yellow()
        );
        println!("{}\n", format!("Cores: {}", num_cpus::get()).yellow());
    }
}

fn main() {
    let mut info: bool = false;
    for argument in args() {
        if argument == "--info" {
            info = true;
        }
    }

    println!("Digite o número do problema ou x para sair: ");
    loop {
        let mut system = sysinfo::System::new();
        system.refresh_all();

        let mut problem_number = String::new();
        io::stdin()
            .read_line(&mut problem_number)
            .expect("Valor Inválido!");

        if problem_number.contains("x") || problem_number.contains("X") {
            break;
        }

        let number: i32 = problem_number.trim().parse::<i32>().unwrap_or(0);
        if number == 0 {
            println!("Valor inválido!");
            println!("Digite o número do problema ou x para sair: ");
            continue;
        }
        let title = get_title(number);

        let now = Instant::now();
        let resultado: i64 = get_result(number);
        let tempo = now.elapsed();

        print!("\x1B[2J\x1B[1;1H");
        print_color(number, title, resultado, tempo, info);

        println!("Digite o número do problema ou x para sair: ");
    }
}
