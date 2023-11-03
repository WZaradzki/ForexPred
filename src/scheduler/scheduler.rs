use crate::scheduler::jobs::print_index_job::run_print_index_job;
use chrono::Utc;
use tokio_schedule::{every, Job};

pub async fn start_scheduler() {
    let print_index_job = every(5)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| (async { run_print_index_job() }));

    actix_rt::spawn(print_index_job);
}
