use std::collections::HashMap;
use std::env;
use std::process;
use std::process::Stdio;
use std::time;

struct TimeResult {
    time: f32,
    exit_code: i32,
}

const EXECUTIONS: i32 = 10;

fn execute(program: String, args: &[String]) -> Vec<TimeResult> {
    let mut results = vec![];

    for _ in 0..EXECUTIONS {
        let now = time::Instant::now();

        let exit_code = process::Command::new(program.clone())
            .args(args)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .unwrap()
            .code()
            .unwrap();

        let time_elapsed = now.elapsed();

        let result = TimeResult {
            exit_code,
            time: time_elapsed.as_secs_f32(),
        };

        results.push(result);
    }
    return results;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[1].to_string();
    let program_args = &args[2..];

    let results = execute(program, program_args);

    let min = results
        .iter()
        .map(|result| result.time)
        .reduce(f32::min)
        .unwrap();

    let max = results
        .iter()
        .map(|result| result.time)
        .reduce(f32::max)
        .unwrap();

    let average = results
        .iter()
        .map(|result| result.time)
        .into_iter()
        .sum::<f32>()
        / results.len() as f32;

    let mut exit_code_count: HashMap<i32, i32> = HashMap::new();

    for result in results {
        let count = exit_code_count.entry(result.exit_code).or_insert(0);
        *count += 1;
    }

    if exit_code_count
        .keys()
        .any(|exit_code| exit_code.to_owned() != 0)
    {
        for (exit_code, count) in exit_code_count.iter() {
            println!(
                "warning: the program has finished with the exit code {exit_code} {count} times"
            )
        }

        println!();
    }

    println!("the min was {min}s");
    println!("the max was {max}s");
    println!("the avg was {average}s");
}
