use fltk::{prelude::*, *};

pub(crate) struct MainWindow {
    window: window::Window,
    task_browser: browser::MultiBrowser,
    task_browser_task_length: i32,
    pub show_detail_button: button::Button,
    pub downloading_checkbutton: button::CheckButton,
    pub waiting_checkbutton: button::CheckButton,
    pub finished_checkbutton: button::CheckButton,
    pub start_button: button::Button,
    pub pause_button: button::Button,
    pub remove_button: button::Button,
    pub add_link_button: button::Button,
    pub add_torrent_button: button::Button,
    pub link_input_dialog: crate::dialog::LinkInputDialog,
}

impl MainWindow {
    pub fn new() -> Self {
        let main_window = window::Window::default()
            .with_size(720, 400)
            .with_label("Poor Downloader");
        let mut app_group = group::Pack::new(0, 0, 720 - 20, 400 - 20, None).center_of_parent();
        app_group.set_spacing(5);
        
        let mut status_group = group::Pack::default().with_size(600, 25).with_type(group::PackType::Horizontal);
        status_group.set_spacing(5);
        let add_link_button = button::Button::new(0, 0, 80, 25, "Add Link");
        let add_torrent_button =button::Button::new(0, 0, 90, 25, "Add Torrent");
        frame::Frame::new(0, 0, 70, 25, "Task Filter:").with_align(enums::Align::Inside | enums::Align::Left);
        let downloading_checkbutton = button::CheckButton::new(0, 0, 100, 25, "Downloading");
        let waiting_checkbutton = button::CheckButton::new(0, 0, 70, 25, "Waiting");
        let finished_checkbutton = button::CheckButton::new(0, 0, 70, 25, "Finished");
        status_group.end();

        let mut task_browser = browser::MultiBrowser::new(0, 0, 600, 300, None);
        task_browser.set_column_widths(&[200, 80, 70, 80, 80, 50, 80]);
        task_browser.set_column_char('\t');
        task_browser.add("NAME\tSIZE\tSTATUS\tD_SPEED\tU_SPEED\tLEFT\tGID");
        
        let mut opration_group = group::Pack::default().with_size(600, 25).with_type(group::PackType::Horizontal);
        opration_group.set_spacing(5);
        let start_button = button::Button::new(0, 0, 60, 25, "Start");
        let pause_button = button::Button::new(0, 0, 60, 25, "Pause");
        let remove_button = button::Button::new(0, 0, 60, 25, "Remove");
        let show_detail_button = button::Button::new(0, 0, 100, 25, "Show Details");
        button::Button::new(0, 0, 100, 25, "Global Setting");
        opration_group.end();

        app_group.end();

        main_window.end();

        let link_input_dialog = crate::dialog::LinkInputDialog::default();

        Self {
            window: main_window,
            task_browser,
            task_browser_task_length: 0,
            show_detail_button,
            downloading_checkbutton,
            waiting_checkbutton,
            finished_checkbutton,
            start_button,
            pause_button,
            remove_button,
            link_input_dialog,
            add_link_button,
            add_torrent_button
        }

    }

    pub fn show(&mut self) {
        self.window.show()
    }

    pub fn deactive_all_status_checkbutton(&mut self) {
        self.downloading_checkbutton.deactivate();
        self.finished_checkbutton.deactivate();
        self.waiting_checkbutton.deactivate();
    }

    pub fn active_all_status_checkbutton(&mut self) {
        self.downloading_checkbutton.activate();
        self.finished_checkbutton.activate();
        self.waiting_checkbutton.activate();
    }

    pub fn update_task(&mut self, status_vec: Vec<aria2_ws::response::Status>) {
        let task_length = status_vec.len().try_into().unwrap();
        let browser_length = self.task_browser_task_length;
        if browser_length <= task_length {
            let mut index = 0;
            for status in status_vec {
                let task_text = status_to_task_text(&status);
                if index < browser_length {
                    self.task_browser.set_text(index + 2, &task_text);
                } else {
                    self.task_browser.add(&task_text);
                }
                index = index + 1;
            }
        } else {
            let mut index = 0;
            for status in status_vec {
                let task_text = status_to_task_text(&status);
                self.task_browser.set_text(index + 2, &task_text);
                index = index + 1;
            }
            for _ in task_length..browser_length {
                self.task_browser.remove(task_length + 2);
            }
        }
        self.task_browser_task_length = task_length;
    }
    
    pub fn show_detail(&self) {
        
    }

    pub fn get_selected_tasks(&self) -> Vec<String> {
        let mut gid_vec = Vec::new();
        for line in self.task_browser.selected_items() {
            if line != 1 {
                let items = self.task_browser.text(line).unwrap();
                let gid = items.split('\t').last().unwrap();
                gid_vec.push(gid.to_string());
            }            
        }
        gid_vec
    }

    pub fn get_status_config(&self) -> StatusConfig {
        let downloading = self.downloading_checkbutton.is_checked();
        let waiting = self.waiting_checkbutton.is_checked();
        let finished = self.finished_checkbutton.is_checked();
        StatusConfig { downloading, waiting, finished }
    }

}

#[derive(Default, Debug)]
pub struct StatusConfig {
    pub downloading: bool,
    pub waiting: bool,
    pub finished: bool
}

fn status_to_task_text(status: &aria2_ws::response::Status) -> String {
    // "NAME\tSIZE\tSTATUS\tD_SPEED\tU_SPEED\tLEFT\tGID"
    let name = {
        if status.files.len() > 1 {
            return format!("Multifiles [{}]", status.files[0].path);
        }
        status.files[0].path.to_owned()
    };

    let size = status.total_length;
    let task_status = match status.status {
        aria2_ws::response::TaskStatus::Active => "Active",
        aria2_ws::response::TaskStatus::Complete => "Completed",
        aria2_ws::response::TaskStatus::Error => "Error",
        aria2_ws::response::TaskStatus::Paused => "Paused",
        aria2_ws::response::TaskStatus::Removed => "Removed",
        aria2_ws::response::TaskStatus::Waiting => "Waiting"
    };

    let download_speed = status.download_speed;
    let upload_speed = status.upload_speed;

    let progress = {
        if status.total_length != 0 {
            status.completed_length * 100 / status.total_length
        } else {
            0
        }
    };

    let gid = status.gid.clone();
    
    format!("{name}\t{size}\t{task_status}\t{download_speed}\t{upload_speed}\t{progress}%\t{gid}")
}