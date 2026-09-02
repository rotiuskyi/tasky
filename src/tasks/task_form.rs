use iced::widget::{button, column, container, row, text, text_editor, text_input};
use iced::{Alignment, Element, Length};

use crate::models::task::{Priority, Task};
use crate::tasks::TITLE_SIZE;
use crate::widgets::select::select;

const PRIORITY_OPS: [Priority; 3] = [Priority::High, Priority::Medium, Priority::Low];

#[derive(Debug, Default)]
pub struct TaskForm {
    title: String,
    description: text_editor::Content,
    priority: Priority,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeTitle(String),
    ChangeDescription(text_editor::Action),
    ChangePriority(Priority),
    Create,
}

pub enum Action {
    None,
    Create(Task),
}

impl TaskForm {
    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::ChangeTitle(title) => self.title = title,
            Message::ChangeDescription(action) => self.description.perform(action),
            Message::ChangePriority(priority) => self.priority = priority,
            Message::Create => {
                if self.title.trim().is_empty() {
                    return Action::None;
                }

                // Taking the whole form both harvests the draft and resets it,
                // so a new field can never be left behind on the next task.
                return Action::Create(std::mem::take(self).into());
            }
        }

        Action::None
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Create task").size(TITLE_SIZE),
            text_input("Title", &self.title)
                .on_input(Message::ChangeTitle)
                .on_submit(Message::Create),
            text_editor(&self.description)
                .placeholder("Description")
                .on_action(Message::ChangeDescription),
            row![
                select(PRIORITY_OPS, Some(&self.priority), Message::ChangePriority)
                    .placeholder("Priority"),
                container(button("Create").on_press(Message::Create))
                    .width(Length::Fill)
                    .align_x(Alignment::End),
            ],
        ]
        .spacing(8)
        .into()
    }
}

impl From<TaskForm> for Task {
    fn from(form: TaskForm) -> Self {
        Task {
            title: form.title,
            description: form.description.text(),
            priority: form.priority,
            is_done: false,
        }
    }
}
