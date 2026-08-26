use iced::Length::Fill;
use iced::alignment::Alignment;
use iced::widget::{button, checkbox, column, container, row, text, text_input};

use crate::icons;

#[derive(Debug, Default, Clone)]
pub struct App {
    task_draft: String,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Noop,
    TaskDraft(String),
    TaskNew(String),
    TaskDone(usize, bool),
    TaskRemove(usize),
}

#[derive(Debug, Default, Clone)]
struct Task {
    title: String,
    done: bool,
}

impl App {
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::TaskDraft(task_draft) => {
                self.task_draft = task_draft;
            }
            Msg::TaskNew(title) => {
                self.tasks.push(Task { title, done: false });
                self.task_draft.clear();
            }
            Msg::TaskDone(i, done) => {
                self.tasks[i].done = done;
            }
            Msg::TaskRemove(i) => {
                self.tasks.remove(i);
            }
            Msg::Noop => {}
        }
    }

    pub fn view(&self) -> iced::Element<'_, Msg> {
        let mut task_list = column![
            text("Task list").size(28),
            row![
                text_input("Create a new task...", &self.task_draft)
                    .on_input(Msg::TaskDraft)
                    .on_submit(self.create_task_from_draft()),
                button("Create").on_press(self.create_task_from_draft()),
            ]
            .spacing(4),
        ]
        .align_x(Alignment::Start)
        .max_width(640)
        .spacing(8)
        .padding(4);

        task_list = task_list.extend(self.tasks.iter().enumerate().map(|(i, t)| {
            row![
                checkbox(t.done)
                    .width(Fill)
                    .label(&t.title)
                    .on_toggle(move |checked| Msg::TaskDone(i, checked)),
                button(icons::trash())
                    .style(button::warning)
                    .on_press(Msg::TaskRemove(i)),
            ]
            .spacing(4)
            .into()
        }));

        container(task_list).align_x(Alignment::Center).into()
    }

    fn create_task_from_draft(&self) -> Msg {
        if self.task_draft.is_empty() {
            return Msg::Noop;
        }
        Msg::TaskNew(self.task_draft.clone())
    }
}
