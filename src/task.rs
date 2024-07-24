

pub struct Task{
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool
}

impl Task{
    pub fn new(id: i32, title: String, description: String, completed: bool) -> Task{
        Task{
            id,
            title,
            description,
            completed
        }
    }

    pub fn complete(&mut self){
        self.completed = true;
    }
}