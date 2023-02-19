use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut main_window = window::Window::default()
        .with_size(720, 400)
        .with_label("Poor Downloader");

    let mut app_group = group::Pack::new(0, 0, 720 - 20, 400 - 20, None).center_of_parent();
    app_group.set_spacing(5);
    
    let mut status_group = group::Pack::default().with_size(600, 25).with_type(group::PackType::Horizontal);
    status_group.set_spacing(5);
    button::Button::new(0, 0, 80, 25, "Add HTTP");
    button::Button::new(0, 0, 90, 25, "Add Torrent");
    frame::Frame::new(0, 0, 70, 25, "Task Filter:").with_align(enums::Align::Inside | enums::Align::Left);
    button::CheckButton::new(0, 0, 40, 25, "All");
    button::CheckButton::new(0, 0, 100, 25, "Downloading");
    button::CheckButton::new(0, 0, 70, 25, "Waiting");
    button::CheckButton::new(0, 0, 70, 25, "Finished");
    status_group.end();

    let mut task_browser = browser::MultiBrowser::new(0, 0, 600, 300, None);
    task_browser.set_column_widths(&[200, 60, 70, 80, 80, 50, 80]);
    task_browser.set_column_char('\t');
    task_browser.add("NAME\tSIZE\tSTATUS\tD_SPEED\tU_SPEED\tLEFT\tGID");

    let mut opration_group = group::Pack::default().with_size(600, 25).with_type(group::PackType::Horizontal);
    opration_group.set_spacing(5);
    button::Button::new(0, 0, 80, 25, "Select All");
    button::Button::new(0, 0, 60, 25, "Start");
    button::Button::new(0, 0, 60, 25, "Pause");
    button::Button::new(0, 0, 60, 25, "Stop");
    button::Button::new(0, 0, 100, 25, "Show Details");
    button::Button::new(0, 0, 100, 25, "Global Setting");
    opration_group.end();

    app_group.end();

    main_window.end();
    main_window.show();
    app.run().unwrap();

}

