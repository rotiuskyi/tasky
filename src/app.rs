use iced::Color;
use iced::Length;
use iced::alignment::Alignment;
use iced::widget::container;
use iced::widget::{
    button, center_x, checkbox, column, hover, right_center, row, scrollable, text, text_editor,
    text_input,
};

use crate::icons;
use crate::widgets::area::{self, area};

const TITLE_SIZE: u32 = 28;

#[derive(Debug, Default, Clone)]
pub struct App {
    task_draft: String,
    description: text_editor::Content,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Noop,
    TaskTitleDraft(String),
    TaskNew(String),
    TaskDone(usize, bool),
    TaskRemove(usize),
    TaskDescriptionDraft(text_editor::Action),
}

#[derive(Debug, Clone)]
struct Task {
    title: String,
    priority: Priority,
    description: String,
    done: bool,
}

#[derive(Debug, Clone)]
enum Priority {
    High,
    Medium,
    Low,
}

impl App {
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::TaskTitleDraft(task_draft) => {
                self.task_draft = task_draft;
            }
            Msg::TaskNew(title) => {
                self.tasks.push(Task {
                    title,
                    priority: Priority::Medium,
                    description: String::from(""),
                    done: false,
                });
                self.task_draft.clear();
                self.description = text_editor::Content::new();
            }
            Msg::TaskDone(i, done) => {
                self.tasks[i].done = done;
            }
            Msg::TaskRemove(i) => {
                self.tasks.remove(i);
            }
            Msg::TaskDescriptionDraft(action) => {
                self.description.perform(action);
            }
            Msg::Noop => {}
        }
    }

    pub fn view(&self) -> iced::Element<'_, Msg> {
        let task_form = column![
            text("Create task").size(TITLE_SIZE),
            text_input("Title", &self.task_draft)
                .on_input(Msg::TaskTitleDraft)
                .on_submit(self.create_task_from_draft()),
            text_editor(&self.description)
                .placeholder("Description")
                .on_action(Msg::TaskDescriptionDraft),
            container(button("Create").on_press(self.create_task_from_draft()))
                .width(Length::Fill)
                .align_x(Alignment::End),
        ]
        .spacing(8);

        let mut task_list = column![text("Task list").size(TITLE_SIZE)];
        task_list = task_list
            .extend(self.tasks.iter().enumerate().map(|(i, t)| {
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
            }))
            .spacing(4);

        scrollable(center_x(
            column![task_form, task_list]
                .align_x(Alignment::Start)
                .max_width(640)
                .spacing(8)
                .padding(4),
        ))
        .into()
    }

    fn create_task_from_draft(&self) -> Msg {
        if self.task_draft.is_empty() {
            return Msg::Noop;
        }
        Msg::TaskNew(self.task_draft.clone())
    }
}
