mod document;
mod paragraphe;

use document::{Document, ajouter_paragraphe_valide};
use paragraphe::{Paragraphe, StyleParagraphe};

fn main() {
    let mut doc = Document::nouveau();

    doc.ajouter_paragraphe(Paragraphe::nouveau("Titre niveau 1", StyleParagraphe::Titre1));
    doc.ajouter_paragraphe(Paragraphe::nouveau("Titre niveau 2", StyleParagraphe::Titre2));
    doc.ajouter_paragraphe(Paragraphe::nouveau("Citation", StyleParagraphe::Citation));
    doc.ajouter_paragraphe(Paragraphe::nouveau("Texte normal", StyleParagraphe::Normal));

    doc.afficher();
    doc.obtenir_paragraphe(0);
    doc.obtenir_paragraphe(99);

    match ajouter_paragraphe_valide(&mut doc, "Bonjour", StyleParagraphe::Normal) {
        Ok(msg) => println!("✓ {}", msg),
        Err(e) => println!("✗ {}", e),
    }

    match ajouter_paragraphe_valide(&mut doc, "", StyleParagraphe::Normal) {
        Ok(msg) => println!("✓ {}", msg),
        Err(e) => println!("✗ {}", e),
    }
}