use crate::document::Document;
use crate::paragraphe::{Paragraphe, StyleParagraphe};
use iced::alignment::Horizontal;
use iced::widget::text_input;
use iced::widget::{button, container, scrollable, text, Column, Row};
use iced::{Color, Length};
use iced::{Element, Task, Theme};

pub fn run() -> iced::Result {
    iced::application("Skribi", Skribi::update, Skribi::view)
        .theme(Skribi::theme)
        .run_with(|| (Skribi::new(), Task::none()))
}

struct Skribi {
    document: Document,
    contenu_brut: String,
    style_actuel: StyleParagraphe,
}

// Les messages — ce que l'utilisateur peut faire
#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    ContentSubmit,
    StyleChanged(StyleParagraphe),
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
            style_actuel: StyleParagraphe::Normal,
        }
    }

    // Modifie l'état selon le message
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ContentChanged(content) => self.contenu_brut = content,
            Message::ContentSubmit => {
                self.document
                    .paragraphes
                    .push(Paragraphe::nouveau(&self.contenu_brut, self.style_actuel));
                self.contenu_brut = String::new();
            }
            Message::StyleChanged(style) => self.style_actuel = style,
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
            let size = match p.style {
                StyleParagraphe::Titre1 => 24.0,
                StyleParagraphe::Normal => 16.0,
                StyleParagraphe::Titre2 => 20.0,
                StyleParagraphe::Citation => 16.0,
            };
            elements.push(text(&p.contenu).size(size).into());
        }

        let button_titre1 = if self.style_actuel == StyleParagraphe::Titre1 {
            button("Titre1")
        } else {
            button("Titre 1").on_press(Message::StyleChanged(StyleParagraphe::Titre1))
        };

        let button_titre2 = if self.style_actuel == StyleParagraphe::Titre2 {
            button("Titre2")
        } else {
            button("Titre 2").on_press(Message::StyleChanged(StyleParagraphe::Titre2))
        };

        let button_citation = if self.style_actuel == StyleParagraphe::Citation {
            button("Citation")
        } else {
            button("Citation").on_press(Message::StyleChanged(StyleParagraphe::Citation))
        };

        let button_normal = if self.style_actuel == StyleParagraphe::Normal {
            button("Normal")
        } else {
            button("Normal").on_press(Message::StyleChanged(StyleParagraphe::Normal))
        };

        Column::new()
            .push(
                Row::new()
                    .push(button_titre1)
                    .push(button_titre2)
                    .push(button_citation)
                    .push(button_normal),
            )
            .push(scrollable(
                container(
                    container(Column::with_children(elements).padding(40))
                        .style(|_| iced::widget::container::Style {
                            background: Some(iced::Background::Color(Color::WHITE)),
                            ..Default::default()
                        })
                        .width(595.0)
                        .height(842.0)
                        .padding(60),
                )
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.25, 0.25, 0.25))),
                    ..Default::default()
                })
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .padding(40),
            ))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}
