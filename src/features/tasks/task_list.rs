use iced::widget::{button, checkbox, column, container, hover, right_center, row, space, text};
use iced::{Alignment, Color, Element, Length};

use crate::features::TITLE_SIZE_MD;
use crate::features::app::TaskList;
use crate::features::tasks::{PRIORITY_OPS, priority_icon};
use crate::icons;
use crate::models::task::Priority;
use crate::widgets::expandable::expandable;
use crate::widgets::hoverable::{self, hoverable};
use crate::widgets::menu::menu;
use crate::widgets::menu_item::{menu_item, menu_item_icon};

#[derive(Debug, Clone)]
pub enum Message {
    Noop,
    Select(usize, bool),
    ChangeStatus(usize, bool),
    ChangePriority(usize, Priority),
    RemoveTask(usize),
    RemoveAll,
}

pub fn update(task_list: &mut TaskList, msg: Message) {
    match msg {
        Message::Select(i, is_checked) => {
            if !task_list.items[i].is_checked {
                task_list.selected_count += 1;
            } else {
                task_list.selected_count -= 1;
            }
            task_list.items[i].is_checked = is_checked;
        }
        Message::ChangeStatus(i, is_done) => {
            task_list.items[i].is_done = is_done;
        }
        Message::ChangePriority(i, p) => task_list.items[i].priority = p,
        Message::RemoveTask(i) => {
            task_list.items.remove(i);
        }
        Message::RemoveAll => {
            task_list.items.retain(|t| !t.is_checked);
            task_list.selected_count = 0;
        }
        // The menu bar captures the press of its trigger, so this one is
        // only ever built, never delivered.
        Message::Noop => {}
    }
}

pub fn task_list(task_list_data: &TaskList) -> Element<'_, Message> {
    let mut task_list = column![
        row![
            text("Task list").size(TITLE_SIZE_MD),
            space().width(Length::Fill),
            row![
                text(format!("Selected: {}", task_list_data.selected_count)),
                menu(
                    menu_item_icon(icons::ellipsis_vertical).on_press(Message::Noop),
                    vec![
                        menu_item("Delete selected")
                            .icon_right(icons::trash)
                            .on_press(Message::RemoveAll)
                    ]
                ),
            ]
            .align_y(Alignment::Center)
            .spacing(8)
        ]
        .align_y(Alignment::Center)
    ];

    task_list = task_list
        .extend(task_list_data.items.iter().enumerate().map(|(i, t)| {
            let task_menu = menu(
                menu_item_icon(icons::ellipsis_vertical).on_press(Message::Noop),
                vec![
                    menu_item(t.priority)
                        .icon_right(icons::list_chevrons_up_down)
                        .on_press(Message::Noop)
                        .with_menu(
                            PRIORITY_OPS
                                .into_iter()
                                .map(|p| {
                                    menu_item(p.to_string())
                                        .icon_left(move || priority_icon(p))
                                        .on_press(Message::ChangePriority(i, p))
                                })
                                .collect(),
                        ),
                    menu_item(display_status(t.is_done))
                        .icon_right(icons::list_chevrons_up_down)
                        .on_press(Message::Noop)
                        .with_menu(vec![
                            menu_item("Open").on_press(Message::ChangeStatus(i, false)),
                            menu_item("Done").on_press(Message::ChangeStatus(i, true)),
                        ]),
                    menu_item("Delete")
                        .icon_right(icons::trash)
                        .on_press(Message::RemoveTask(i)),
                ],
            );

            // An invisible twin keeps the room for the trigger in the layout,
            // so the row does not resize once it shows up under the cursor.
            let task_menu_placeholder =
                button(icons::ellipsis_vertical()).style(|_theme, _status| button::Style {
                    text_color: Color::TRANSPARENT,
                    ..button::Style::default()
                });

            expandable(
                hoverable(
                    container(hover(
                        row![
                            priority_icon(t.priority),
                            checkbox(t.is_checked)
                                .label(format!("[{}] {}", display_status(t.is_done), &t.title))
                                .on_toggle(move |checked| Message::Select(i, checked)),
                            task_menu_placeholder,
                        ]
                        .align_y(Alignment::Center)
                        .width(Length::Fill)
                        .spacing(12),
                        right_center(task_menu),
                    ))
                    .padding([4, 8]),
                )
                .style(hoverable::card),
                container(text(&t.description)).padding([4, 8]),
            )
            .into()
        }))
        .spacing(4);

    task_list.into()
}

fn display_status(is_done: bool) -> &'static str {
    match is_done {
        true => "Done",
        false => "Open",
    }
}
