use todo_cli::to_do_list::ToDoList;

//command line app used for todo list
fn main(){
    println!("Welcome to the todo list app");
    println!("==============================");
    let mut _list = ToDoList::new();
    loop{
        println!();
        println!("What would you like to do?");
        println!("(1) Add a task");
        println!("(2) Complete a task");
        println!("(3) Remove a task");
        println!("(4) List tasks");
        println!("(5) Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input{
            "1" => {
                println!();
                println!("Enter the title of the task:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();
                let title = title.trim();

                println!("Enter the description of the task:");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).unwrap();
                let description = description.trim();

                _list.add_task(title.to_string(), description.to_string());
            },
            "2" => {
                println!();
                println!("Enter the id of the task to complete:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: i32 = id.trim().parse().unwrap();
                _list.complete_task(id);
            },
            "3" => {
                println!();
                println!("Enter the id of the task to remove:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: i32 = id.trim().parse().unwrap();
                _list.remove_task(id);
            },
            "4" => {
                println!();
                for task in &_list.tasks{
                    let status = if task.completed {"Completed"} else {"Not completed"};
                    println!("ID: {}, Title: {}, Description: {}, Status: {}", task.id, task.title, task.description, status);
                }
            },
            "5" => {
                break;
            },
            _ => {
                println!("Invalid input");
            }
        }
    }

}
