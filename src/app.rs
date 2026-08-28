use iced::Color;
use iced::Length;
use iced::alignment::Alignment;
use iced::widget::{
    button, center_x, checkbox, column, hover, right_center, row, scrollable, text, text_input,
};

use crate::icons;
use crate::widgets::area::{self, area};

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
            let remove_btn = button(icons::trash())
                .style(button::warning)
                .on_press(Msg::TaskRemove(i));

            // An invisible twin keeps the room for the button in the layout, so
            // the row does not resize once the button shows up under the cursor.
            let remove_btn_placeholder =
                button(icons::trash()).style(|_theme, _status| button::Style {
                    text_color: Color::TRANSPARENT,
                    ..button::Style::default()
                });

            area(hover(
                row![
                    checkbox(t.done)
                        .width(Length::Fill)
                        .label(&t.title)
                        .on_toggle(move |checked| Msg::TaskDone(i, checked)),
                    remove_btn_placeholder,
                ]
                .align_y(Alignment::Center)
                .spacing(4),
                right_center(remove_btn),
            ))
            .padding([4, 8])
            .style(area::card)
            .into()
        }));

        scrollable(center_x(task_list)).into()
    }

    fn create_task_from_draft(&self) -> Msg {
        if self.task_draft.is_empty() {
            return Msg::Noop;
        }
        Msg::TaskNew(self.task_draft.clone())
    }
}
