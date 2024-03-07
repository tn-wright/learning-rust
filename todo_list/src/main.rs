use dialoguer::Input;
use dialoguer::FuzzySelect;
use console::Style;
use console::style;
use std::fmt;

struct Task {
    name: String,
    completed: bool
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    const OPERATIONS: [&str; 9] = ["show","add","complete","edit","delete","clean","empty","help","exit"];

    loop {
        //Get the current command from the user
        let command_input = FuzzySelect::new().with_prompt("Choose an operation").items(&OPERATIONS).interact().unwrap();

        match command_input {
            0 => show_tasks(&tasks),
            1 => add_task(&mut tasks),
            2 => complete_task(&mut tasks),
            3 => edit_task(&mut tasks),
            4 => delete_task(&mut tasks),
            5 => remove_completed_tasks(&mut tasks),
            6 => clear_tasks(&mut tasks),
            7 => help(),
            8 => break,
            9_usize.. => panic!("Invalid operator supplied"),
        }
    }
}

fn show_tasks(tasks: &Vec<Task>) {
    if tasks.len() == 0 {
        println!("{}", style("You have no tasks").blue());
        return;
    }

    for task in tasks {
        let style = if task.completed { Style::new().green().strikethrough() } else { Style::new().red() };
        println!("{}", style.apply_to(&task.name));
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let new_task = Input::new().with_prompt("Enter task name").interact_text().unwrap();
    let new_task: Task = Task {
        name: new_task,
        completed: false
    };
    tasks.push(new_task);
}

fn complete_task(tasks: &mut Vec<Task>) {
    let selected_task = FuzzySelect::new().with_prompt("Which task would you like to complete?").items(&tasks).interact().unwrap();
    tasks[selected_task].completed = true;
}

fn edit_task(tasks: &mut Vec<Task>) {
    let selected_task = FuzzySelect::new().with_prompt("Which task would you like to edit?").items(&tasks).interact().unwrap();
    let new_task = Input::new().with_prompt("Enter new task name").interact_text().unwrap();
    tasks[selected_task].name = new_task;
}

fn delete_task(tasks: &mut Vec<Task>) {
    let selected_task = FuzzySelect::new().with_prompt("Which task would you like to delete?").items(&tasks).interact().unwrap();
    tasks.remove(selected_task);
}

fn remove_completed_tasks(tasks: &mut Vec<Task>) {
    tasks.retain(|task| task.completed == false);
    println!("{}", style("Removed all completed tasks").blue());
}

fn clear_tasks(tasks: &mut Vec<Task>) {
    tasks.clear();
    println!("{}", style("All tasks removed").blue());
}

fn help() {
    println!("{}", style("A simple CLI tool for managing a To-Do list").blue());  
    println!();
    println!("{}", style("Show: Show all tasks. Incomplete tasks are shown in red while complete tasks are shown in green and struck through.").blue());
    println!();
    println!("{}", style("Add: Add a new task to the list.").blue());
    println!();
    println!("{}", style("Complete: Mark a task as completed.").blue());
    println!();
    println!("{}", style("Edit: Change the name of a task.").blue());
    println!();
    println!("{}", style("Delete: Remove a task from the list.").blue());
    println!();
    println!("{}", style("Clean: Remove all completed tasks from the list.").blue());
    println!();
    println!("{}", style("Empty: Remove all tasks from the list.").blue());
}
