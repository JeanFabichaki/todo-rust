use std::io;
use std::process;
use std::io::ErrorKind;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;

use colored::*;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool 
}

fn main() {

    initialize_file();

    let contents = fs::read_to_string("todo.json")
    .expect("Something went wrong reading the file todo.json");

    let mut todos;

    if contents.is_empty() {
        todos = Vec::new();
    }else{
        
        let f = File::open("todo.json").unwrap();
        let reader = BufReader::new(f);

        let t:Vec<Todo> = serde_json::from_reader(reader).unwrap();
        todos = t;

    }

    print_todos(&todos);

    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line");

        let command_parts: Vec<&str> = command.split_whitespace().collect();

        match command_parts.len() {
            0 => invalid_command(&command),
            1 => match command_parts[0] {
                "list" => print_todos(&todos),
                "close" => process::exit(0),
                "help" => print_help(),
                "trash" => print_todos_deleted(&todos),
                _ => invalid_command(&command),
            },
            _ => match command_parts[0] {
                "add" => add_todo(&mut todos, &command_parts[1..].join(" ")),
                "remove" => {
                    if let Ok(num) = command_parts[1].parse::<i16>() {
                        remove_todo(&mut todos, num)
                    }
                }
                "done" => {
                    if let Ok(num) = command_parts[1].parse::<i16>() {
                        mark_done(&mut todos, num)
                    }
                },
                "undone" => {
                    if let Ok(num) = command_parts[1].parse::<i16>() {
                        mark_undone(&mut todos, num)
                    }
                },
                "recover" => {
                    if let Ok(num) = command_parts[1].parse::<i16>() {
                        recover_todo(&mut todos, num)
                    }
                }
                "edit" => {
                    if let Ok(num) = command_parts[1].parse::<i16>(){
                        edit_todo(&mut todos, num, &command_parts[2..].join(" "))
                    }
                }
                _ => invalid_command(&command),
            },
        }

        match command_parts[0] == "list" {
            false => print_todos(&todos),
            _ => ()  
        }
        
    }
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) {
    let new_id = todos.len() as i16 + 1;

    todos.push(Todo {
        id: new_id,
        title: title.to_string(),
        completed: false,
        deleted: false,
    });

    write_in_file(todos);
}

fn remove_todo(todos: &mut Vec<Todo>, todo_id: i16) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
        todo.deleted = true;
    }

    write_in_file(todos);
}

fn mark_done(todos: &mut Vec<Todo>, todo_id: i16) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
        todo.completed = true;
    }

    write_in_file(todos);
}

fn mark_undone(todos: &mut Vec<Todo>, todo_id: i16) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
        todo.completed = false;
    }

    write_in_file(todos);
}

fn edit_todo(todos: &mut Vec<Todo>, todo_id: i16, new_title: &str){
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id){
        todo.title = new_title.to_string();
    } 

    write_in_file(todos);   
}

fn recover_todo(todos: &mut Vec<Todo>, todo_id: i16) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
        todo.deleted = false;
    }

    write_in_file(todos);
}

fn print_todos(todos: &Vec<Todo>){
    println!("\n\n{}\n----------------","Todo List:".blue().bold());

    for todo in todos {
        if !todo.deleted {
            let done = if todo.completed {"✔"} else {""};
            println!("[{}] {} {}", done.green(), todo.id, todo.title);
        }
    }
}

fn print_todos_deleted(todos: &Vec<Todo>){
    println!("\n\n{}\n----------------","Todo deleted List:".red().bold());

    for todo in todos {
        if todo.deleted {
            let done = if todo.completed {" ✔ "} else {""};
            println!("[{}] {} {}", done.green(), todo.id, todo.title);
        }
    }
}

fn print_help(){
    println!("\n\n{}\n","Commands:".bold());
    println!("{} <title> : add new todo", "add    ".blue());
    println!("{} : list task", "list   ".blue());
    println!("{} <number> : mark done todo", "done   ".blue());
    println!("{} <number> : remove todo", "remove ".blue());
    println!("{} <number> : recover deleted todo", "recover".blue());
    println!("{} <number> <title> : edit todo", "edit   ".blue());
    println!("{} : list deleted todo", "trash  ".blue());
    println!("{} : exit", "close  ".blue());
}

fn invalid_command(command : &str){
    println!("Invalid command: {}", command.red());
    println!("Try typing : {}", "help".green());
}

fn initialize_file(){
    
    let f = File::open("todo.json");
    
    match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("todo.json"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            olther_error => panic!("Problem opening the file: {:?} ", olther_error)
        }        
    };
}


fn write_in_file(todo_list: &Vec<Todo>){

    let file = OpenOptions::new()
    .write(true)
    .truncate(true)    
    .open("todo.json")
    .unwrap();

    serde_json::to_writer_pretty(&file, &todo_list).unwrap();

}