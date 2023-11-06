pub trait JobTrait {
    fn run(&self);

    fn get_name(&self) -> String;
}

pub struct Job {
    pub name: String,
    pub description: String,
}

impl Job {
    pub fn new(name: String, description: String) -> Job {
        Job { name, description }
    }
}
