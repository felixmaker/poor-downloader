use fltk::prelude::WidgetExt;
use anyhow::anyhow;
use tokio::io::AsyncBufReadExt;

pub mod window;
pub mod dialog;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateTask(Vec<aria2_ws::response::Status>),
    ShowDetail
}

#[tokio::main]
async fn main() {   

    let app = fltk::app::App::default();
    let (s, mut r) = fltk::app::channel();
    let mut main_window = crate::window::MainWindow::new();
    main_window.show();
    
    main_window.show_detail_button.emit(s.clone(), Message::ShowDetail);

    tokio::spawn(async move {
        let client = aria2_ws::Client::connect("ws://127.0.0.1:6800/jsonrpc", None)
            .await.unwrap();
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        for _ in 0..1000 {
            interval.tick().await;
            let active_status_vec = client.tell_active().await.unwrap();
            // println!("{:?}", active_status_vec);
            s.send(Message::UpdateTask(active_status_vec));
        }
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::UpdateTask(status_vec) => { main_window.update_task(status_vec) }
                Message::ShowDetail => { main_window.show_detail() }
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
        Err(_) => { Err(anyhow!("failed to run aria2c")) }
    }
}