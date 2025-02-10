use bincode::{self};
use iced::widget::{button, column, text, text_input, Column};
use serde::{Deserialize, Serialize};
use std::fs;

fn main() -> iced::Result {
    iced::application("Flow. Saga. Sunrise.", Flow::update, Flow::view)
        .window_size((300.0, 500.0))
        .centered()
        .run()
    // iced::run("Flow. Saga. Sunrise.", Flow::update, Flow::view)
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
}

#[derive(Default)]
struct Flow {
    input_take_name: String,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
enum Message {
    CreateTask,
    InputChanged(String),
}

//bincode

impl Flow {
    fn update(&mut self, message: Message) {
        let enc = fs::read("tasks.bin");
        if enc.is_ok() {
            let (decoded, _bytes_read): (Vec<Task>, usize) =
                bincode::serde::decode_from_slice(&enc.unwrap(), bincode::config::standard())
                    .unwrap();
            self.tasks = decoded;
        }

        match message {
            Message::CreateTask => {
                self.tasks.push(Task {
                    name: self.input_take_name.clone(),
                });
                self.input_take_name.clear();

                let result =
                    bincode::serde::encode_to_vec(&self.tasks, bincode::config::standard())
                        .unwrap();
                fs::write("tasks.bin", &result).expect("failed tro write");
            }
            Message::InputChanged(cont) => {
                self.input_take_name = cont;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let col = column![
            text_input("Task name", &self.input_take_name).on_input(Message::InputChanged),
            button("Create").on_press(Message::CreateTask)
        ];

        col.extend(
            self.tasks
                .iter()
                .map(|t| iced::Element::from(text(&t.name)))
                .into_iter(),
        )
    }
}
