use std::cmp;

use iced::event::{self, Event};
use iced::keyboard::{self, key};
use iced::widget::text_input::Id;
use iced::widget::{button, column, container, text, text_input};
use iced::{executor, window, Background, Color};
use iced::{Application, Command, Element, Settings, Subscription, Theme};
use uuid::Uuid;

const SELECTED_TODO_BACKGROUND: Color = Color::from_rgb(0.9, 0.9, 0.9);
const HOVERED_TODO_BACKGROUND: Color = Color::from_rgb(0.8, 0.9, 0.9);

pub fn main() -> iced::Result {
    TodoApp::run(Settings::default())
}

#[derive(Debug, Default)]
struct TodoApp {
    value: i32,
    text_input: String,
    todos: Vec<Todo>,
    editing_uuid: Option<Uuid>,
    selected_index: Option<usize>,
}

impl TodoApp {
    fn check_editing(&mut self) {
        if self.editing_uuid.is_some() {
            let id = self.editing_uuid.unwrap();

            if self.selected_index.is_some() {
                let todo_id = self.todos[self.selected_index.unwrap()].id;
                if todo_id != id {
                    self.editing_uuid = None;
                }
            }
        }
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
struct Todo {
    title: String,
    description: String,
    id: Uuid,
    priority: i32,
}

impl Todo {
    fn new() -> Self {
        Self {
            title: "New Todo".to_owned(),
            description: "New Todo Description".to_owned(),
            id: Uuid::new_v4(),
            priority: 0,
        }
    }

    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("todo-{i}"))
    }
}

// Define the SelectedTodo style
pub struct SelectedTodo;

impl button::StyleSheet for SelectedTodo {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(SELECTED_TODO_BACKGROUND)),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(HOVERED_TODO_BACKGROUND)),
            ..button::Appearance::default()
        }
    }
}

impl From<SelectedTodo> for iced::theme::Button {
    fn from(val: SelectedTodo) -> Self {
        Self::Custom(Box::new(val))
    }
}

// impl Into<button::Button<'static, Message>> for SelectedTodo {
//     fn into(self) -> button::Button<'static, Message> {
//         button::Button::new(Text::new("Selected Todo")).style(self)
//     }
// }

#[derive(Debug, Clone)]
enum Message {
    ExitApp,
    EventOccurred(Event),
    TextInputChanged(String),
    AddNewTodo,
    BeginEditTodo(Uuid),
    FinishEditTodo(Uuid),
    TodoTextInputChanged(String),
    TryEditCurrentItem,
}

trait MyApplication {
    fn handle_key_press(&mut self, key: iced::keyboard::Key) -> Command<Message>;
}

impl MyApplication for TodoApp {
    fn handle_key_press(&mut self, key: iced::keyboard::Key) -> Command<Message> {
        match key.as_ref() {
            keyboard::Key::Named(key::Named::ArrowUp) | key::Key::Character("k") => {
                if (self.todos.len() == 0) {
                    return Command::none();
                }

                if self.selected_index.is_none() {
                    self.selected_index = Some(0);
                } else if self.selected_index.unwrap() > 0 {
                    // Prevent underflow
                    self.selected_index = Some(self.selected_index.unwrap() - 1);
                }
                println!("Selected index: {:?}", self.selected_index);

                self.check_editing();
                Command::none()
            }
            keyboard::Key::Named(key::Named::ArrowDown) | key::Key::Character("j") => {
                if (self.todos.len() == 0) {
                    return Command::none();
                }

                if self.selected_index.is_none() {
                    self.selected_index = Some(0);
                } else {
                    self.selected_index = Some(cmp::min(
                        self.selected_index.unwrap() + 1,
                        self.todos.len() - 1,
                    ));
                }
                println!("Selected index: {:?}", self.selected_index);

                self.check_editing();
                Command::none()
            }
            key::Key::Character("i") => {
                Command::perform(async {}, |()| Message::TryEditCurrentItem)
            }
            keyboard::Key::Named(key::Named::Escape) => {
                println!("Escape key pressed");
                Command::perform(async {}, |()| Message::ExitApp)
            }
            keyboard::Key::Named(key::Named::Enter) => {
                println!("Enter key pressed");
                if self.editing_uuid.is_some() {
                    let id = self.editing_uuid.unwrap();
                    Command::perform(async {}, move |()| Message::FinishEditTodo(id.clone()))
                } else {
                    Command::perform(async {}, |()| Message::AddNewTodo)
                }
            }
            _ => Command::none(),
        }
    }
}

impl Application for TodoApp {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn theme(&self) -> iced::theme::Theme {
        return Theme::custom(
            "Main".to_owned(),
            iced::theme::palette::Palette {
                background: iced::Color::from_rgb8(224, 228, 204),
                text: iced::Color::from_rgb8(167, 219, 216),
                primary: iced::Color::from_rgb8(105, 210, 231),
                success: iced::Color::from_rgb8(243, 134, 48),
                danger: iced::Color::BLACK,
            },
        );
    }

    fn title(&self) -> String {
        String::from("icedtodo")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ExitApp => return window::close(window::Id::MAIN),
            Message::TextInputChanged(msg) => {
                println!("Text input changed: ");
                println!("{}", msg);
                self.text_input = msg;
            }
            Message::EventOccurred(event) => match event {
                #[allow(unused_variables)]
                Event::Mouse(mouse) => {}
                Event::Keyboard(keyboard) => match keyboard {
                    iced::keyboard::Event::KeyPressed { key, .. } => {
                        let cloned_event = key.clone();
                        println!("{:?}", cloned_event);
                        return self.handle_key_press(key);
                    }
                    _ => {}
                },
                _ => {
                    let cloned_event = event.clone();
                    println!("{:?}", cloned_event);
                }
            },
            Message::AddNewTodo => {
                self.todos.push(Todo::new());
            }
            Message::BeginEditTodo(uuid) => {
                println!("Editing todo: {:?}", uuid);
                self.editing_uuid = Some(uuid);
            }
            Message::FinishEditTodo(uuid) => {
                println!("Finished editing todo: {:?}", uuid);
                self.editing_uuid = None;
            }
            Message::TodoTextInputChanged(msg) => {
                println!("Todo text input changed: {}", msg);
                if self.editing_uuid.is_some() {
                    let uuid = self.editing_uuid.unwrap();
                    for todo in &mut self.todos {
                        if todo.id == uuid {
                            todo.title = msg.clone();
                        }
                    }
                } else {
                    panic!("No todo is being edited");
                }
            }
            Message::TryEditCurrentItem => {
                if self.selected_index.is_some() {
                    let index = self.selected_index.unwrap();
                    let id = self.todos[index].id;
                    self.editing_uuid = Some(id);
                    return text_input::focus(Todo::text_input_id(index));
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut app = column![].spacing(10);

        for (index, todo) in self.todos.iter().enumerate() {
            let mut content = container(text(todo.title.clone()));
            if self.editing_uuid.is_some() && self.editing_uuid.unwrap() == todo.id {
                content = container(
                    text_input("Type here", &todo.title)
                        .id(Todo::text_input_id(index))
                        .on_input(|x| -> Message { Message::TodoTextInputChanged(x) })
                        .on_submit(Message::FinishEditTodo(todo.id))
                        .padding(10)
                        .size(20),
                );
            }
            let mut todo_view = button(content).on_press(Message::BeginEditTodo(todo.id));
            if self.selected_index == Some(index) {
                todo_view = todo_view.style(SelectedTodo);
            }
            app = app.push(todo_view);
        }

        let new_todo_btn = button("+").on_press(Message::AddNewTodo);
        app = app.push(new_todo_btn);

        container(app).padding(20).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }
}
