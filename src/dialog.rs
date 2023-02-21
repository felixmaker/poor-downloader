use fltk::{prelude::*, *};

pub struct LinkInputDialog {
    url_input: input::MultilineInput,
    window: window::Window,
}

impl LinkInputDialog {
    pub fn default() -> Self {

        let mut main_window = window::Window::default()
            .with_size(300, 240)
            .with_label("Add Http Dialog");

        let mut main_group = group::Pack::new(0, 0, 300 - 20, 240 - 20, None).center_of_parent();
        main_group.set_spacing(5);

        frame::Frame::new(0, 0, 300, 20, "Enter URL link:").with_align(enums::Align::Left | enums::Align::Inside);
        let url_input = input::MultilineInput::new(0, 0, 300, 160, None);

        let mut button_group = group::Pack::new(0, 0, 300, 25, None).with_type(group::PackType::Horizontal);
        button_group.set_spacing(5);
        button::Button::new(0, 0, 80, 25, "Submit");
        button::Button::new(0, 0, 80, 25, "Cancel").set_callback({
            let mut window = main_window.clone();
            move |_| {
                window.hide();
            }
        });
        button_group.end();

        main_group.end();

        main_window.end();
        main_window.make_modal(true);
        // main_window.show();

        while main_window.shown() {
            app::wait();
        }

        Self { url_input, window: main_window }

    }

    pub fn value(&self) -> String {
        self.url_input.value()
    }

    pub fn show(&mut self) {
        self.window.show();
    }

    pub fn hide(&mut self) {
        self.window.hide();
    }
}

struct TorrentInputDialog { }

impl TorrentInputDialog {
    fn default() -> Self {
        let mut main_window = window::Window::default()
        .with_size(400, 360)
        .with_label("Add Http Dialog");

        let mut main_group = group::Pack::new(0, 0, 400 - 20, 360 - 20, None).center_of_parent();
        main_group.set_spacing(5);

        frame::Frame::new(0, 0, 400, 20, "Input a torrent file:").with_align(enums::Align::Left | enums::Align::Inside);
        
        let mut file_input_group = group::Flex::new(0, 0, 400, 25, None).with_type(group::FlexType::Row);
        input::Input::default();
        let browser_file_button = button::Button::default().with_label("Browser");
        file_input_group.set_pad(5);
        file_input_group.set_size(&browser_file_button, 70);
        file_input_group.end();

        frame::Frame::new(0, 0, 400, 20, "Select files (Left browser means need):").with_align(enums::Align::Left | enums::Align::Inside);

        let mut torrent_browser_group = group::Flex::new(0, 0, 400, 200, None).with_type(group::PackType::Horizontal);
        let mut torrent_browser = browser::MultiBrowser::default().with_label("(Files need)");
        torrent_browser.set_column_widths(&[120, 50]);
        torrent_browser.set_column_char('\t');
        torrent_browser.add("NAME\tSIZE");

        let mut torrent_button_group = group::Pack::default();
        torrent_button_group.set_spacing(5);
        button::Button::new(0, 0, 20, 20, "<");
        button::Button::new(0, 0, 20, 20, ">");
        torrent_button_group.end();

        let mut torrent_browser = browser::MultiBrowser::default().with_label("(Files not need)");
        torrent_browser.set_column_widths(&[120, 50]);
        torrent_browser.set_column_char('\t');
        torrent_browser.add("NAME\tSIZE");

        torrent_browser_group.set_pad(5);
        torrent_browser_group.set_size(&torrent_button_group, 20);
        torrent_browser_group.end();

        frame::Frame::new(0, 0, 400, 15, None);

        let mut button_group = group::Pack::new(0, 0, 300, 25, None).with_type(group::PackType::Horizontal);
        button_group.set_spacing(5);
        button::Button::new(0, 0, 80, 25, "Submit");
        button::Button::new(0, 0, 80, 25, "Cancel").set_callback({
            let mut window = main_window.clone();
            move |_| {
                window.hide();
            }
        });
        button_group.end();

        main_group.end();

        main_window.end();
        main_window.make_modal(true);
        main_window.show();

        while main_window.shown() {
            app::wait();
        }

        Self {  }
    }
}


pub struct DetailDialog {}
