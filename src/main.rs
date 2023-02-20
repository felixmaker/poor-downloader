use std::collections::HashMap;

use anyhow::{Result, anyhow};
use aria2_ws::Client;
use fltk::{prelude::*, *};
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() {      

    let app = app::App::default();
    let (app_sender, mut app_receiver) = tokio::sync::mpsc::channel(32);

    let mut main_window = window::Window::default()
        .with_size(720, 400)
        .with_label("Poor Downloader");

    let mut app_group = group::Pack::new(0, 0, 720 - 20, 400 - 20, None).center_of_parent();
    app_group.set_spacing(5);
    
    let mut status_group = group::Pack::default().with_size(600, 25).with_type(group::PackType::Horizontal);
    status_group.set_spacing(5);
    button::Button::new(0, 0, 80, 25, "Add Link").set_callback(move |_| {
        LinkDialog::default();
    });
    button::Button::new(0, 0, 90, 25, "Add Torrent").set_callback(move |_| {
        TorrentDialog::default();
    });
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

    // communicate with aria2 rpc
    tokio::spawn(async move {
        match start_aria2().await {
            Ok(mut aria2) => {
                let client = Client::connect("ws://127.0.0.1:6800/jsonrpc", None)
                    .await.unwrap();                
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
                for _ in 0..1000 {
                    interval.tick().await;
                    let active_status_vec = client.tell_active().await.unwrap();
                    println!("{:?}", active_status_vec);
                    app_sender.send(Message::UpdateTask(active_status_vec)).await.unwrap();
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    });
    
    while app.wait() {
        if let Ok(msg) = app_receiver.try_recv() {
            match msg {
                Message::UpdateTask(status_vec) => {
//task_browser.add("NAME\tSIZE\tSTATUS\tD_SPEED\tU_SPEED\tLEFT\tGID");
                    
                    let mut gids: Vec<String> = Vec::new();

                    for i in task_browser.selected_items() {
                        if i == 1 {
                            continue;
                        }
                        if let Some(text) = task_browser.text(i) {
                            let items: Vec<&str> = text.split('\t').collect();
                            if let Some(gid) = items.last() {
                                gids.push(gid.to_string());
                            }
                        }
                    }

                    task_browser.clear();
                    task_browser.add("NAME\tSIZE\tSTATUS\tD_SPEED\tU_SPEED\tLEFT\tGID");
                    let mut index = 2;
                    for status in status_vec {
                        let gid = status.gid.clone();
                        task_browser.add(&status_to_task_text(&status));
                        task_browser.select(index);
                        index = index + 1;
                    }
                }
            }
        }
    }

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
        _ => "Other"
    };

    let download_speed = status.download_speed;
    let upload_speed = status.upload_speed;

    let progress = {
        if status.total_length != 0 {
            status.completed_length / status.total_length * 100
        } else {
            0
        }
    };

    let gid = status.gid.clone();
    
    format!("{name}\t{size}\t{task_status}\t{download_speed}\t{upload_speed}\t{progress}%\t{gid}")
}

#[derive(Debug)]
enum Message {
    UpdateTask(Vec<aria2_ws::response::Status>)
}

struct LinkDialog {
    url_input: input::MultilineInput
}

impl LinkDialog {
    fn default() -> Self {

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
        main_window.show();

        while main_window.shown() {
            app::wait();
        }

        Self { url_input }

    }

    fn value(&self) -> String {
        self.url_input.value()
    }
}


struct TorrentDialog {

}

impl TorrentDialog {
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

async fn start_aria2() -> Result<tokio::process::Child> {
    let mut aria2 = tokio::process::Command::new("aria2c")
        .args(&["--enable-rpc"])        
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
        
    let stdout = aria2.stdout.take()
        .ok_or_else(|| anyhow!("Failed to take aria2 stdout"))?;

    let reader = tokio::io::BufReader::new(stdout);
    let mut lines = reader.lines();
    let res = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        while let Some(line) = lines.next_line().await? {
            if line.contains("IPv4 RPC: listening on TCP") {
                return anyhow::Ok::<()>(());
            }
        }
        Err(anyhow!("failed to run aria2c"))
    });

    match res.await? {
        Ok(_) => { Ok(aria2) }
        Err(_) => { Err(anyhow!("failed to run aria2c")) }
    }
    
}
