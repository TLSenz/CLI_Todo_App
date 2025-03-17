
use std::io::{stdin, BufReader};
use std::num::ParseIntError;
use std::fs::File;
use std::io::{self,BufRead};
use std::io::ErrorKind::FileTooLarge;

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    loop {
        let choice = menu();
        match choice {
            1 => create_todo(&mut todo_list),
            2 => see_todos(&todo_list),
            3 => mark_todo_as_done(&mut todo_list),
            4 => {
                let buffer_list = import_todos();
                for n in buffer_list.into_iter() {
                    todo_list.push(n);
                }

            }
            5 => break,  // Exit the loop
            _ => println!("Invalid option, please try again."),
        }
    }
}
struct Todo{
    name : String,
    todo : String,

}
fn menu() -> i32{
    println!("Was moechtest du machen");

    println!("1. Hinzufuegen");
    println!("2.Anschauen");
    println!("3.Task Loeschen");
    println!("5.Export Tasks");
    println!("5.Import Tasks");

    let mut user_input:String = String::new();

    stdin().read_line(&mut user_input).expect("Coud not Read Commandline");

    let zahl: Result<i32, ParseIntError> = user_input.trim().parse::<i32>();
    match zahl {
        Ok(num) => {
            println!("Parsed {} sucessfull",num );
            num
        }
        Err(e) => {
            println!("Cant be Parsed, Error: {}", e);
            0
        }
    }
}
fn see_todos(todo_list: &Vec<Todo>){
    for n in todo_list.iter(){
        println!("{}",n.name);
        println!("{}",n.todo);

    }
}
fn mark_todo_as_done( todo_list: & mut Vec<Todo>){
    for (index , todo) in todo_list.iter().enumerate(){
        println!("{}, {}", index, todo.todo)
    }
    println!("Please Select the Todo by Index, it will be Removed");
    let selected_todo:i32;

    let mut  input:String = String::new();

    stdin().read_line(&mut input).expect("Could not read Input");

    let input = input.trim();

    selected_todo = input.parse::<i32>().unwrap();

    if selected_todo < 0 ||  selected_todo > todo_list.len() as i32 {
        0;
    }
    else {
        todo_list.remove(selected_todo as usize);

    }
}

fn create_todo( todo_list:&mut Vec<Todo>){
    println!("Waht is the Name of your Todo");
    let mut todo_name:String = String::new();
    stdin().read_line(&mut todo_name).expect("Could not Read Input");
    println!("What is the description of your Todo ?");
    let mut  todo_description:String = String::new();
    stdin().read_line(&mut todo_description).expect("Could not Read Input");

    let todo = Todo{
        name: String::from(todo_name),
        todo: String::from(todo_description),

    };
    todo_list.push(todo);


}
fn import_todos() -> Vec<Todo>{
    let result = read_todos_from_file();

    let todo_in_string_format: Vec<String> =  match result {
        Ok(todo_in_string_format) => todo_in_string_format,
        Err(error) => {
            println!("Error while reading file: {}",error);
            Vec::new()
        }
    };
    let todos:Vec<Todo> = todo_in_string_format
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(",").collect()
            if parts.len() == 2{
                Todo{
                    name : parts[0].to_string(),
                    todo : parts[1].to_string()
                }
            }
            else {
                Todo{
                    name: String::new(),
                    todo : String::new()

                }
            }
        })
    .collect();

    todos
}
fn read_todos_from_file() -> Vec<String>{
    let file = File::open(r"C:\Users\zempsv\Documents\RustTodo.txt");
    let reader =   io::BufReader::new(file);
    let mut file_input:Vec<String> = Vec::new();


    for line in reader.lines(){
        let line = line?;
        file_input.push(line);
    }
    file_input
}