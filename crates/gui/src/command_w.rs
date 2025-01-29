use iced::{
    widget::{button, column, horizontal_rule, row, scrollable, text, text_input, Column},
    Element,
    Length::Fill,
};

#[derive(Default, Debug)]
pub struct State {
    messages: Vec<Message>,
    cmd_input: String,
}

// A message from client to server (Request)
// or from server to client (Response).
#[derive(Clone, Debug)]
enum Message {
    Command(String),
    Response(String),
}

#[derive(Debug, Clone)]
pub enum Msg {
    CommandInputChanged(String),
    SendCommand,
}

impl State {
    pub fn view(&self) -> Element<Msg> {
        let messages = scrollable(self.view_messages()).width(Fill).height(Fill);

        let cmd_input = text_input("Command", &self.cmd_input)
            .on_input(Msg::CommandInputChanged)
            .on_submit(Msg::SendCommand)
            .width(Fill);

        let cmd_button = button(">").on_press(Msg::SendCommand);

        column![messages, row![cmd_input, cmd_button].padding(5).spacing(5)]
            .padding(5)
            .spacing(5)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn view_messages(&self) -> Element<Msg> {
        let mut col = Column::new();

        let mut is_first = true;
        for message in &self.messages {
            let (sender, txt) = match message {
                Message::Command(txt) => ("CLIENT", txt),
                Message::Response(txt) => ("SERVER", txt),
            };

            if is_first {
                is_first = false;
            } else {
                col = col.push(horizontal_rule(2));
            }

            col = col.push(text(format!("[{}]: {}", sender, txt)).width(Fill));
        }

        col.width(Fill).spacing(1).into()
    }

    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::CommandInputChanged(cmd) => self.cmd_input = cmd,
            Msg::SendCommand => {
                let cmd = self.cmd_input.clone();
                if cmd.is_empty() {
                    return;
                }
                self.cmd_input.clear();
                self.messages.push(Message::Command(cmd.clone()));
                // TODO: actually send command
                // TODO: delete dummy (next line):
                self.messages
                    .push(Message::Response(format!("Response to cmd: {cmd}")));
            }
        }
    }
}
