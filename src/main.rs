use todo_cli::to_do_list::ToDoList;

fn main() {
   let mut list = ToDoList::new();
    list.add_task("Learn Rust".to_string(), "Learn Rust by building projects".to_string());
    list.add_task("Learn Python".to_string(), "Learn Python by building projects".to_string());
    list.add_task("Learn Java".to_string(), "Learn Java by building projects".to_string());
    list.add_task("go running".to_string(), "run with zoe".to_string());

    for task in &list.tasks{
        println!("{}: {}", task.id, task.title);
    }
}
