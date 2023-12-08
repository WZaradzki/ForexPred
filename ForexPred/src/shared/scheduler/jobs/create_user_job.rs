// use crate::{
//     models, repository,
//     scheduler::jobs::base::JobTrait,
// };

// struct CreateUserJob {
//     name: String,
//     description: String,
// }

// impl CreateUserJob {
//     fn new(name: String, description: String) -> CreateUserJob {
//         CreateUserJob { name, description }
//     }
// }

// pub fn run_create_user_job() -> () {
//     let job = CreateUserJob::new(
//         "PrintIndexJob".to_string(),
//         "This is print index job for test purpose".to_string(),
//     );

//     impl JobTrait for CreateUserJob {
//         fn run(&self) {
//             let db = repository::database::Database::new();

//             let user = models::users::User {
//                 id: uuid::Uuid::new_v4().to_string(),
//                 name: "Test User".to_string(),
//                 description: None,
//                 created_at: None,
//                 updated_at: None,
//             };

//             let user = db.create_user(user).expect("ERROR: Failed to create user");

//             println!("User created: {:?}", user);
//         }

//         fn get_name(&self) -> String {
//             self.name.clone()
//         }
//     }

//     return job.run();
// }
