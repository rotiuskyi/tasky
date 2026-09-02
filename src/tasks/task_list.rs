use iced::Alignment;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::widget::{button, checkbox, column, hover, right_center, row, text};
use iced_aw::menu::{Item, Menu, MenuBar};

use crate::icons;
use crate::models::task::Task;
use crate::tasks::TITLE_SIZE;
use crate::widgets::area::{self, area};

#[derive(Debug, Clone)]
pub enum Message {
    Noop,
    ChangeStatus(usize, bool),
    RemoveTask(usize),
}

/// Applies a message of the list to the tasks it was built from.
pub fn update(tasks: &mut Vec<Task>, msg: Message) {
    match msg {
        Message::ChangeStatus(i, is_done) => tasks[i].is_done = is_done,
        Message::RemoveTask(i) => {
            tasks.remove(i);
        }
        // The menu bar captures the press of its trigger, so this one is
        // only ever built, never delivered.
        Message::Noop => {}
    }
}

pub fn task_list(tasks: &[Task]) -> Element<'_, Message> {
    let mut task_list = column![text("Task list").size(TITLE_SIZE)];
    task_list = task_list
        .extend(tasks.iter().enumerate().map(|(i, t)| {
            let task_menu = MenuBar::new(vec![Item::with_menu(
                button(icons::ellipsis_vertical())
                    .style(button::subtle)
                    // The bar captures the press itself, so this message never
                    // fires; it only keeps the trigger from looking disabled.
                    .on_press(Message::Noop),
                Menu::new(vec![
                    Item::new(
                        button(row![icons::trash(), text("Remove")].spacing(8))
                            .style(button::subtle)
                            .width(Length::Fill)
                            .on_press(Message::RemoveTask(i)),
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
                    checkbox(t.is_done)
                        .label(&t.title)
                        .on_toggle(move |checked| Message::ChangeStatus(i, checked)),
                    task_menu_placeholder,
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill)
                .spacing(4),
                right_center(task_menu),
            ))
            .padding([4, 8])
            .style(area::card)
            .into()
        }))
        .spacing(4);

    task_list.into()
}
