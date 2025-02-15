use iced::widget::{button, checkbox, column, keyed_column, row, text_input, Column, Row};
use serde::{Deserialize, Serialize};

fn main() {
    iced::application(Flow::title, Flow::update, Flow::view)
        .centered()
        .run_with(Flow::new)
        .unwrap()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    content: String,
}

impl Task {
    fn get_title(&self) -> String {
        self.content.clone()
    }

    fn view(&self) -> Row<Message> {
        row![checkbox(&self.content, false)]
    }
}

#[derive(Default)]
struct Flow {
    task_input_content: String,
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    CreateTaskClicked,
    StateSaved(Result<(), &'static str>),
    StateLoaded(Result<State, &'static str>),
    InputChanged(String),
}

//bincode

const DATA_PATH: &'static str = "./state.bin";

impl Flow {
    fn new() -> (Self, iced::Task<Message>) {
        (
            Flow::default(),
            iced::Task::perform(State::load(String::from(DATA_PATH)), Message::StateLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Flow")
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::StateLoaded(load_r) => {
                self.state = load_r.unwrap();
                println!("Tasks: {:?}", self.state.tasks);
            }
            Message::InputChanged(content) => self.task_input_content = content,
            Message::CreateTaskClicked => {
                if self.task_input_content.len() > 0 {
                    self.state.add_task(self.task_input_content.clone());
                    self.task_input_content = String::new();

                    return iced::Task::perform(
                        self.state.clone().save(String::from(DATA_PATH)),
                        Message::StateSaved,
                    );
                }
            }
            Message::StateSaved(result) => match result {
                Ok(()) => println!("saved ok"),
                Err(msg) => println!("saved not ok {}", msg),
            },
            // _ => {}
        }

        iced::Task::none()
    }

    fn view(&self) -> Column<Message> {
        column![
            text_input("Task name", &self.task_input_content).on_input(Message::InputChanged),
            button("Create").on_press(Message::CreateTaskClicked),
            column(self.state.tasks.iter().map(|t| t.view().into()))
        ]
    }
}

#[derive(Default, Debug, Clone)]
struct State {
    tasks: Vec<Task>,
}

impl State {
    fn add_task(&mut self, content: String) -> String {
        let task = Task { content: content };
        let title = task.get_title();
        self.tasks.push(task);

        return String::from(title);
    }

    async fn save(self, path: String) -> Result<(), &'static str> {
        use async_std::prelude::*;

        let mut file = async_std::fs::File::create(path)
            .await
            .map_err(|_| "failed create")?;

        let buf = bincode::serde::encode_to_vec(&self.tasks, bincode::config::standard())
            .map_err(|_| "failed encode")?;
        file.write_all(&buf).await.map_err(|_| "failed write")?;

        Ok(())
    }

    async fn load(path: String) -> Result<State, &'static str> {
        use async_std::prelude::*;

        let mut file = async_std::fs::File::open(path)
            .await
            .map_err(|_| "open create")?;

        let mut buf = Vec::new();

        file.read_to_end(&mut buf)
            .await
            .map_err(|_| "failed read")?;

        let (tasks, _): (Vec<Task>, usize) =
            bincode::serde::decode_from_slice(&buf, bincode::config::standard())
                .map_err(|_| "failed to decode")?;

        Ok(State { tasks: tasks })
    }
}
