mod features;
mod models;

use iced::alignment::Alignment;
use iced::widget::{center_x, column, scrollable};

use crate::app::features::tasks::task_form::{self, TaskForm};
use crate::app::features::tasks::task_list::{self, TaskList, task_list};

#[derive(Debug, Default)]
pub struct App {
    task_form: TaskForm,
    task_list: TaskList,
}

#[derive(Debug, Clone)]
pub enum Message {
    TaskForm(task_form::Message),
    TaskList(task_list::Message),
}

impl App {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::TaskForm(msg) => match self.task_form.update(msg) {
                task_form::Action::None => {}
                task_form::Action::Create(task) => self.task_list.items.push(task),
            },
            Message::TaskList(msg) => task_list::update(&mut self.task_list, msg),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let task_form = self.task_form.view().map(Message::TaskForm);
        let task_list = task_list(&self.task_list).map(Message::TaskList);

        scrollable(center_x(
            column![task_form, task_list]
                .align_x(Alignment::Start)
                .max_width(640)
                .spacing(8)
                .padding([28, 4]),
        ))
        .into()
    }
}
