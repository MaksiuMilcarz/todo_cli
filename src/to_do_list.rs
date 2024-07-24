use crate::task::Task;

pub struct ToDoList{
    pub tasks: Vec<Task>
}

impl ToDoList{
    pub fn new() -> ToDoList{
        ToDoList{
            tasks: Vec::new()
        }
    }

    pub fn add_task(&mut self, title: String, description: String){
        let id = self.tasks.len() as i32;
        let task = Task::new(id, title, description, false);
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, id: i32){
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id){
            task.complete();
        }
    }

    pub fn remove_task(&mut self, id: i32){
        self.tasks.retain(|t| t.id != id);
    }
}

