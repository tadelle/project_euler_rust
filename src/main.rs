mod problems;

use colored::*;
use num_cpus;
use problems::{get_result, get_title};
use std::env::args;
use std::io;
use std::time::{Duration, Instant};
use sysinfo::SystemExt;

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
