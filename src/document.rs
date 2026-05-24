use crate::paragraphe::{Paragraphe, StyleParagraphe};

pub struct Document {
    pub paragraphes: Vec<Paragraphe>,
}

impl Document {
    pub fn nouveau() -> Document {
        Document {
            paragraphes: Vec::new(),
        }
    }

    pub fn ajouter_paragraphe(&mut self, paragraphe: Paragraphe) {
        self.paragraphes.push(paragraphe);
    }

    pub fn afficher(&self) {
        for paragraphe in &self.paragraphes {
            paragraphe.afficher();
        }
    }

    pub fn obtenir_paragraphe(&self, index: usize) {
        match self.paragraphes.get(index) {
            Some(p) => p.afficher(),
            None => println!("Aucun paragraphe à l'index {}", index),
        }
    }
}

pub fn ajouter_paragraphe_valide(
    doc: &mut Document,
    contenu: &str,
    style: StyleParagraphe,
) -> Result<String, String> {
    if contenu.is_empty() {
        Err(String::from("Le contenu ne peut pas être vide"))
    } else {
        doc.ajouter_paragraphe(Paragraphe::nouveau(contenu, style));
        Ok(String::from("Paragraphe ajouté"))
    }
}