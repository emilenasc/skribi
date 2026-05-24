mod document;
mod paragraphe;

use iced::{Element, Task, Theme};
use iced::widget::{column, text};

pub fn main() -> iced::Result {
    iced::application("Skribi", Skribi::update, Skribi::view)
        .theme(Skribi::theme)
        .run()
}

// L'état de l'application
#[derive(Default)]
struct Skribi {
    contenu: String,
}

// Les messages — ce que l'utilisateur peut faire
#[derive(Debug, Clone)]
enum Message {
    Rien,
}

impl Skribi {
    // État initial
    fn new() -> Self {
        Skribi {
            contenu: String::from("Bienvenue dans Skribi !"),
        }
    }

    // Modifie l'état selon le message
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Rien => Task::none(),
        }
    }

    // Dessine l'interface
    fn view(&self) -> Element<'_,Message> {
    column![
        text("Skribi").size(32),
        text("Bienvenue dans Skribi !").size(16),
    ]
    .padding(40)
    .into()
}

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}