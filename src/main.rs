mod document;
mod paragraphe;

use crate::document::Document;
use crate::paragraphe::{Paragraphe, StyleParagraphe};
use iced::widget::text_input;
use iced::widget::{text, Column};
use iced::{Element, Task, Theme};

pub fn main() -> iced::Result {
    iced::application("Skribi", Skribi::update, Skribi::view)
        .theme(Skribi::theme)
        .run_with(|| (Skribi::new(), Task::none()))
}

// L'état de l'application
struct Skribi {
    document: Document,
    contenu_brut: String,
}

// Les messages — ce que l'utilisateur peut faire
#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    ContentSubmit,
}

impl Skribi {
    fn new() -> Self {
        let mut paragraphes: Vec<Paragraphe> = Vec::new();
        paragraphes.push(Paragraphe::nouveau("Titre", StyleParagraphe::Titre1));
        paragraphes.push(Paragraphe::nouveau(
            "Hello, world!",
            StyleParagraphe::Normal,
        ));

        Skribi {
            document: Document { paragraphes },
            contenu_brut: String::new(),
        }
    }

    // Modifie l'état selon le message
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ContentChanged(content) => self.contenu_brut = content,
            Message::ContentSubmit => {
                self.document.paragraphes.push(Paragraphe::nouveau(&self.contenu_brut, StyleParagraphe::Normal));
                self.contenu_brut = String::new();
            }
        };
    
        Task::none()
    }

    // Dessine l'interface
    fn view(&self) -> Element<'_, Message> {
        let mut elements: Vec<Element<'_, Message>> = Vec::new();

          elements.push(
            text_input("placeholder", &self.contenu_brut)
                .on_input(Message::ContentChanged)
                .on_submit(Message::ContentSubmit)
                .into(),
        );

        for p in self.document.paragraphes.iter() {
            elements.push(text(&p.contenu).size(16).into());
        }

        Column::with_children(elements).padding(40).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
