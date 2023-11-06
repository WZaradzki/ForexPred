use crate::scheduler::jobs::base::{Job, JobTrait};

pub fn run_print_index_job() -> () {
    let job = Job::new("PrintIndexJob".to_string(), "This is print index job for test purpose".to_string());

    impl JobTrait for Job {
        fn run(&self) {
            println!("Running job: {} about {}", self.name, self.description);
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    return job.run();
}
