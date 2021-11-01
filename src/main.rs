use std::env;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{BufReader, BufWriter, Write};
use std::process;

struct Todo {
    todo: Vec<String>,
}

impl Todo {
    fn new() -> Result<Self, String> {
        let todos_file: String = String::from(String::from(dirs::data_dir().unwrap().to_str().unwrap()) + "\\Todo\\All.todo");

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(todos_file)
            .expect("Couldn't open the todofile");
        let mut buf_reader = BufReader::new(&todofile);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();
        let todo = contents.to_string().lines().map(str::to_string).collect();
        
        Ok(Self { todo })
    }
    
    fn list(&self) {
        for (number, task) in self.todo.iter().enumerate() {
            if task.len() > 5 {
                let number = (number + 1).to_string();

                let symbol = &task[..4];
                let task = &task[4..];

                if symbol == "[*] " {
                    println!("{} {}", number, task);
                } else if symbol == "[ ] " {
                    println!("{} {}", number, task);
                }
            }
        }
    }
    
    fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("todo raw takes only 1 argument, not {}", arg.len())
        } else if arg.is_empty() {
            eprintln!("todo raw takes 1 argument (done/todo)");
        } else {
            for task in self.todo.iter() {
                if task.len() > 5 {
                    let symbol = &task[..4];
                    let task = &task[4..];
                    
                    if symbol == "[*] " && arg[0] == "done" {
                        println!("{}", task);
                    } else if symbol == "[ ] " && arg[0] == "todo" {
                        println!("{}", task);
                    }
                }
            }
        }
    }
    
    fn add(&self, args: &[String]) {
        let todos_file: String = String::from(String::from(dirs::data_dir().unwrap().to_str().unwrap()) + "\\Todo\\All.todo");

        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }
        
        let todofile = OpenOptions::new()
            .create(true)
            .append(true)
            .open(todos_file)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }
            
            let line = format!("[ ] {}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }
    
    fn remove(&self, args: &[String]) {
        let todos_file: String = String::from(String::from(dirs::data_dir().unwrap().to_str().unwrap()) + "\\Todo\\All.todo");

        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }
        
        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(todos_file)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&"done".to_string()) && &line[..4] == "[*] " {
                continue;
            }
            if args.contains(&(pos + 1).to_string()) {
                continue;
            }

            let line = format!("{}\n", line);

            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }
    
    fn sort(&self) {
        let todos_file: String = String::from(String::from(dirs::data_dir().unwrap().to_str().unwrap()) + "\\Todo\\All.todo");
        let newtodo: String;

        let mut todo = String::new();
        let mut done = String::new();

        for line in self.todo.iter() {
            if line.len() > 5 {
                if &line[..4] == "[ ] " {
                    let line = format!("{}\n", line);
                    todo.push_str(&line);
                } else if &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    done.push_str(&line);
                }
            }
        }

        newtodo = format!("{}{}", &todo, &done);
        
        let mut todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(todos_file)
            .expect("Couldn't open the todo file");
            
        todofile
            .write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
    }

    fn done(&self, args: &[String]) {
        let todos_file: String = String::from(String::from(dirs::data_dir().unwrap().to_str().unwrap()) + "\\Todo\\All.todo");

        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }
        
        let todofile = OpenOptions::new()
            .write(true)
            .open(todos_file)
            .expect("Couldn't open the todofile");
        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if line.len() > 5 {
                if args.contains(&(pos + 1).to_string()) {
                    if &line[..4] == "[ ] " {
                        let line = format!("[*] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    }
                } else if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    buffer
                        .write_all(line.as_bytes())
                        .expect("unable to write data");
                }
            }
        }
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s] 
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX] 
        removes a task
        Example: todo rm 4 
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort 
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";

fn help() {
    println!("{}", TODO_HELP);
}

fn main() {
    let todo = Todo::new().expect("Couldn't create the todo instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" => todo.remove(&args[2..]),
            "done" => todo.done(&args[2..]),
            "raw" => todo.raw(&args[2..]),
            "sort" => todo.sort(),
            "help" | "--help" | "-h" | _ => help(),
        }
    } else {
        todo.list();
    }
}