pub mod task_form;
pub mod task_list;

use iced::widget::text::{self, Text};

use crate::app::models::Priority;
use crate::icon;

pub const PRIORITY_OPTS: [Priority; 3] = [Priority::High, Priority::Medium, Priority::Low];

fn priority_icon<'a>(p: Priority) -> Text<'a> {
    match p {
        Priority::High => icon::circle_alert().style(text::danger),
        Priority::Medium => icon::circle().style(text::warning),
        Priority::Low => icon::circle_dashed().style(text::success),
    }
}
