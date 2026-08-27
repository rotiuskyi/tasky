mod app;
mod icons;
mod widgets;

use app::App;
use iced::theme::Theme;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::CatppuccinLatte)
        .font(icons::FONT)
        .title("Tasky")
        .run()
}
