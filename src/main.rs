/*
use std::fs::ReadDir;
use std::fs;
use std::num::ParseIntError;
fn main() {
    // let result: Result<ReadDir, std::io::Error> = fs::read_dir("C://"); // Result<ReadDir>
    // if result.is_ok() {
    //     for read_dir in result.unwrap() {
    //         println!("{}", read_dir.map(|r| r.file_name().into_string().unwrap())
    //             .unwrap());
    //     }
    // }
    let line = "1\n2\n3\n4\nqqe\n";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{}", n),
            Err(..) => println!("ERRORE")
        }
    }
}*/

use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, Command};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    let button = druid::widget::Button::new("click me")
        .fix_width(TEXT_BOX_WIDTH)
        .on_click(|_ctx, data: &mut HelloState, _c| println!("{}", data.name));

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button);

    // center the two widgets in the available space
    Align::centered(layout) // Senza il ; restituisce questo
}
