pub enum StyleParagraphe {
    Normal,
    Titre1,
    Titre2,
    Citation,
}

pub struct Paragraphe {
    pub contenu: String,
    pub style: StyleParagraphe,
}

impl Paragraphe {
    pub fn nouveau(contenu: &str, style: StyleParagraphe) -> Paragraphe {
        Paragraphe {
            contenu: String::from(contenu),
            style,
        }
    }

    pub fn afficher(&self) {
        match self.style {
            StyleParagraphe::Normal => println!("{}", self.contenu),
            StyleParagraphe::Titre1 => println!("# {}", self.contenu),
            StyleParagraphe::Titre2 => println!("## {}", self.contenu),
            StyleParagraphe::Citation => println!("> {}", self.contenu),
        }
    }
}