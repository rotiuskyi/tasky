use iced::Color;
use iced::Length;
use iced::alignment::Alignment;
use iced::widget::container;
use iced::widget::{
    button, center_x, checkbox, column, hover, right_center, row, scrollable, text, text_editor,
    text_input,
};
use iced_aw::menu::{Item, Menu, MenuBar};

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
    TaskDescriptionDraft(text_editor::Action),
    TaskNew(String),
    TaskDone(usize, bool),
    TaskRemove(usize),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Task {
    title: String,
    priority: Priority,
    description: String,
    done: bool,
}

#[allow(dead_code)]
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
                let task_menu = MenuBar::new(vec![Item::with_menu(
                    button(icons::ellipsis_vertical())
                        .style(button::subtle)
                        // The bar captures the press itself, so this message never
                        // fires; it only keeps the trigger from looking disabled.
                        .on_press(Msg::Noop),
                    Menu::new(vec![
                        Item::new(
                            button(row![icons::trash(), text("Remove")].spacing(8))
                                .style(button::subtle)
                                .width(Length::Fill)
                                .on_press(Msg::TaskRemove(i)),
                        )
                        .close_on_click(true),
                    ])
                    .max_width(160.0)
                    .offset(4.0),
                )]);

                // An invisible twin keeps the room for the trigger in the layout,
                // so the row does not resize once it shows up under the cursor.
                let task_menu_placeholder =
                    button(icons::ellipsis_vertical()).style(|_theme, _status| button::Style {
                        text_color: Color::TRANSPARENT,
                        ..button::Style::default()
                    });

                area(hover(
                    row![
                        checkbox(t.done)
                            .width(Length::Fill)
                            .label(&t.title)
                            .on_toggle(move |checked| Msg::TaskDone(i, checked)),
                        task_menu_placeholder,
                    ]
                    .align_y(Alignment::Center)
                    .spacing(4),
                    right_center(task_menu),
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
