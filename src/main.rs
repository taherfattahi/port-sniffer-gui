
use iced::{button, Align, Button, Column, Element, Sandbox, Settings,
           Text, TextInput, text_input, Length, Container};

use std::net::{IpAddr, TcpStream};
use std::str::FromStr;

use std::sync::mpsc::{Sender, channel};
use std::{thread};

use std::io::{self, Write};


fn scan(tx: Sender<u16>, addr: IpAddr, port: u16) {
    match TcpStream::connect((addr, port)) {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(e) => {
            println!("{}", e);
            println!("Close Port!!!!");
        }
    }
}


#[derive(Default)]
struct PortSniffer {
    text_value: String,
    input_ip_value: String,
    input_port_value: String,

    sniffer_button: button::State,
    input_ip: text_input::State,
    input_port: text_input::State,
}

#[derive(Debug, Clone)]
enum Message {
    SnifferPressed,
    InputIpChanged(String),
    InputPortChanged(String),
}

impl Sandbox for PortSniffer {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Rust GUI - Port - Sniffer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SnifferPressed => {
                match IpAddr::from_str(&self.input_ip_value) {
                    Ok(value) => {
                        self.input_ip_value = value.to_string();

                        let port = self.input_port_value.trim().parse::<u16>().unwrap();
                        let addr = value;
                        let (tx, rx) = channel();

                        let tx1 = tx.clone();
                        thread::spawn(move || {
                            scan(tx1, addr, port);
                        });

                        let mut out = vec![];
                        drop(tx);

                        for p in rx {
                            out.push(p);
                        }

                        out.sort();
                        for v in out {
                            println!("{} is open= ", v);
                            let vo = &*v.to_string();
                            self.text_value = "is open".to_string() + vo;
                        }
                    }
                    Err(_) => {
                        self.input_ip_value = "".to_string();
                        self.text_value = "plz insert correct IP address!!".to_string();
                    }
                }
            }
            Message::InputIpChanged(value) => {
                self.input_ip_value = value.to_string();
            }
            Message::InputPortChanged(value) => {
                match value.trim().parse::<u32>() {
                    Ok(value) => {
                        self.input_port_value = value.to_string();
                    }
                    Err(_) => {
                        self.input_port_value = "".to_string();
                        self.text_value = "plz insert integer type!!".to_string();
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Container::new(content)
        Column::new()
            .padding(10)
            .spacing(10)
            .align_items(Align::Center)
            .push(TextInput::new(&mut self.input_ip,
                                 "IP address",
                                 &self.input_ip_value,
                                 Message::InputIpChanged,
            ).width(Length::Units(200)
            ))
            .push(TextInput::new(&mut self.input_port,
                                 "Port Number",
                                 &self.input_port_value,
                                 Message::InputPortChanged,
            ).width(Length::Units(200))
            )
            .push(
                Button::new(&mut self.sniffer_button,
                            Text::new("Scan!!"))
                    .on_press(Message::SnifferPressed),
            )
            .push(Text::new(self.text_value.to_string()).size(20))
            .into()
    }
}


fn main() {
    PortSniffer::run(Settings::default())
}