use crossterm::{
    cursor::Hide,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io;
use structs::TaskResult;

mod structs;

fn main() -> Result<(), io::Error> {
    execute!(io::stdout(), Hide)?;

    print_result(execute_function1());

    print_result(execute_function2());

    Ok(())
}

fn execute_function1() -> TaskResult {
    TaskResult {
        task_name: "function 1".to_string(),
        description: "task 1".to_string(),
        result: "FAIL".to_string(),
        message: "Test".to_string(),
    }
}

fn execute_function2() -> TaskResult {
    TaskResult {
        task_name: "function 2".to_string(),
        description: "task 1".to_string(),
        result: "PASS".to_string(),
        message: "".to_string(),
    }
}

fn print_result(task_result: TaskResult) {
    if task_result.result == "FAIL" {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Red),
            Print("[✗] "),
            SetForegroundColor(Color::White),
            Print(format!(
                "{} - {}",
                task_result.task_name.clone(),
                task_result.description.clone()
            )),
            SetForegroundColor(Color::Red),
            Print(format!("\n\t{}", task_result.message.clone())),
        )
        .unwrap();
    } else {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print("[✓] "),
            SetForegroundColor(Color::White),
            Print(format!(
                "{} - {}",
                task_result.task_name.clone(),
                task_result.description.clone()
            )),
        )
        .unwrap();
    }

    println!(); // Move to the next line
}
