pub mod task_form;
pub mod task_list;

use iced::widget::Text;
use iced::widget::text;

use crate::icons;
use crate::models::task::Priority;

pub const PRIORITY_OPS: [Priority; 3] = [Priority::High, Priority::Medium, Priority::Low];

fn priority_icon<'a>(p: Priority) -> Text<'a> {
    match p {
        Priority::High => icons::signal_high().style(text::danger),
        Priority::Medium => icons::signal_medium().style(text::warning),
        Priority::Low => icons::signal_low().style(text::success),
    }
}
