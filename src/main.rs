use iced::widget::{button, column, text, text_input, Column};

fn main() -> iced::Result {
    iced::application("Flow. Saga. Sunrise.", Flow::update, Flow::view)
        .window_size((300.0, 500.0))
        .centered()
        .run()
    // iced::run("Flow. Saga. Sunrise.", Flow::update, Flow::view)
}

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

impl Flow {
    fn update(&mut self, message: Message) {
        match message {
            Message::CreateTask => {
                self.tasks.push(Task {
                    name: self.input_take_name.clone(),
                });
                self.input_take_name.clear();
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
