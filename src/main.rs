mod features;
mod icons;
mod models;
mod widgets;

use iced::theme::Theme;

use features::app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::CatppuccinLatte)
        .font(icons::FONT)
        .title("Tasky")
        .run()
}
