use fltk::prelude::WidgetExt;
use anyhow::anyhow;
use tokio::io::AsyncBufReadExt;

pub mod window;
pub mod dialog;

#[derive(Debug, Clone)]
enum Message {
    UpdateTaskBrowser(Vec<aria2_ws::response::Status>),
    ShowDetail,
    UpdateTaskLoop,
    SelectedTask(Operation),
    ShowLinkInputDialog,
    DisplayError(String)
}

#[derive(Debug, Clone)]
enum Operation {
    Pause,
    Unpause,
    Remove
}

#[derive(Debug)]
enum BackgroundTask {
    UpdateStatusConfig(window::StatusConfig),
    Unpause(Vec<String>),
    Remove(Vec<String>),
    Pause(Vec<String>)
}

#[tokio::main]
async fn main() {   

    let app = fltk::app::App::default();
    let (s, r) = fltk::app::channel();

    let (task_sender, mut task_receiver) = 
        tokio::sync::mpsc::unbounded_channel::<BackgroundTask>();
    
    let mut main_window = crate::window::MainWindow::new();

    main_window.finished_checkbutton.emit(s.clone(), Message::UpdateTaskLoop);
    main_window.waiting_checkbutton.emit(s.clone(), Message::UpdateTaskLoop);
    main_window.downloading_checkbutton.emit(s.clone(), Message::UpdateTaskLoop);

    main_window.show_detail_button.emit(s.clone(), Message::ShowDetail);

    main_window.start_button.emit(s.clone(), Message::SelectedTask(Operation::Unpause));
    main_window.pause_button.emit(s.clone(), Message::SelectedTask(Operation::Pause));
    main_window.remove_button.emit(s.clone(), Message::SelectedTask(Operation::Remove));

    main_window.downloading_checkbutton.set_checked(true);
    main_window.waiting_checkbutton.set_checked(true);

    main_window.add_link_button.emit(s.clone(), Message::ShowLinkInputDialog);

    main_window.show();

    tokio::spawn(async move {
        let client = aria2_ws::Client::connect("ws://127.0.0.1:6800/jsonrpc", None)
            .await.unwrap();

        let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));
        let mut status_config = window::StatusConfig::default();
        status_config.downloading = true;
        status_config.waiting = true;
        
        loop {
            interval.tick().await;

            if let Ok(background_task) = task_receiver.try_recv() {
                match background_task {
                    BackgroundTask::UpdateStatusConfig(config) => {
                        status_config = config;
                    }
                    BackgroundTask::Unpause(gid_vec) => {
                        for gid in gid_vec {
                            if let Err(error) = client.unpause(&gid).await {
                                println!("{:?}", error);
                            }
                        }
                    }
                    BackgroundTask::Pause(gid_vec) => {
                        for gid in gid_vec {
                            if let Err(error) = client.pause(&gid).await {

                            }
                        }
                    }
                    BackgroundTask::Remove(gid_vec) => {
                        for gid in gid_vec {
                            if let Err(error) = client.pause(&gid).await {

                            }
                        }
                    }
                }
            }

            let task_status_vec = {                
                let mut vec = Vec::new();
                if status_config.downloading {
                    vec.append(&mut client.tell_active().await.unwrap());
                }
                if status_config.waiting {
                    vec.append(&mut client.tell_waiting(0, 99).await.unwrap());
                }
                if status_config.finished {
                    vec.append(&mut client.tell_stopped(0, 99).await.unwrap());
                }
                vec
            };

            s.send(Message::UpdateTaskBrowser(task_status_vec));
        }
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::UpdateTaskBrowser(status_vec) => {
                    main_window.update_task(status_vec);
                    main_window.active_all_status_checkbutton();
                }
                Message::ShowDetail => { main_window.show_detail() }
                Message::UpdateTaskLoop => {
                    let config = main_window.get_status_config();
                    if let Ok(_) = task_sender.send(BackgroundTask::UpdateStatusConfig(config)) {
                        main_window.deactive_all_status_checkbutton();
                    }
                }
                Message::SelectedTask(Operation) => {
                    let gid_vec = main_window.get_selected_tasks();
                    let result = match Operation {
                        Operation::Pause => task_sender.send(BackgroundTask::Pause(gid_vec)),
                        Operation::Unpause => task_sender.send(BackgroundTask::Unpause(gid_vec)),
                        Operation::Remove => task_sender.send(BackgroundTask::Remove(gid_vec))
                    };
                }
                Message::ShowLinkInputDialog => {
                    main_window.link_input_dialog.show();
                }
                Message::DisplayError(error) => {
                    println!("{}", error);
                }
            }
        }
    }
   
}


async fn start_engine() -> anyhow::Result<tokio::process::Child> {
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
        Err(_) => { Err(anyhow!("timeout")) }
    }
}