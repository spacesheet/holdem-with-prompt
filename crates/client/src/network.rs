use holdem_shared::*;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct NetworkClient {
    tx: Sender<ClientMessage>,
    rx: Receiver<ServerMessage>,
}

impl NetworkClient {
    pub fn connect(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;
        
        let (tx_to_server, rx_from_ui) = channel::<ClientMessage>();
        let (tx_to_ui, rx_to_ui) = channel::<ServerMessage>();

        // 서버로 메시지 전송 스레드
        let mut write_stream = stream.try_clone()?;
        thread::spawn(move || {
            while let Ok(msg) = rx_from_ui.recv() {
                let json = serde_json::to_string(&msg).unwrap() + "\n";
                if write_stream.write_all(json.as_bytes()).is_err() {
                    break;
                }
                let _ = write_stream.flush();
            }
        });

        // 서버로부터 메시지 수신 스레드
        thread::spawn(move || {
            let mut reader = BufReader::new(stream);
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if let Ok(msg) = serde_json::from_str::<ServerMessage>(&line) {
                            let _ = tx_to_ui.send(msg);
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            tx: tx_to_server,
            rx: rx_to_ui,
        })
    }

    pub fn send(&mut self, msg: ClientMessage) {
        let _ = self.tx.send(msg);
    }

    pub fn receive(&mut self) -> Option<ServerMessage> {
        self.rx.try_recv().ok()
    }
}