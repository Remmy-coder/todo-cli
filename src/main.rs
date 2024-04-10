use std::env;

struct TodoItem {
    name: String,
    completed: char,
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name,
            completed: ' ',
        };
    }
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name)
        }
    }

    fn mark_done(&mut self, index: usize) {
        self.list[index].completed = 'x'
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();

    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        "done" => Command::Done(arguments[2].parse().expect("Error converting to integer")),
        _ => panic!("Please provide an accepted argument"),
    };

    todo_list.add_to_list("Say hi to Remmy".to_string());
    todo_list.add_to_list("Do something with Rust.".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        }
    }
}
