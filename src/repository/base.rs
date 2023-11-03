use super::database::Database;

pub struct Repositories {
    // pub user_repository: UserRepository,
    // pub course_repository: CourseRepository,
    // // Add other repositories here...
}

impl Repositories {
    pub fn new() -> Self {
        let database_url = Database::get_database_url();
        // let user_repository = UserRepository::new(database_url.clone());
        // let course_repository = CourseRepository::new(database_url.clone());
        // Initialize other repositories...

        Repositories {
            // user_repository,
            // course_repository,
            // Add other repositories here...
        }
    }
}