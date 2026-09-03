pub mod task_form;
pub mod task_list;

use crate::models::task::Priority;

pub const PRIORITY_OPS: [Priority; 3] = [Priority::High, Priority::Medium, Priority::Low];
