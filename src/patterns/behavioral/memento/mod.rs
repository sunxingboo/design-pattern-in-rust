mod memento;
mod originator;
mod concrete_originator;
mod concrete_memento;

#[cfg(test)]
mod tests {
    use crate::patterns::behavioral::memento::concrete_originator::Editor;
    use crate::patterns::behavioral::memento::originator::Originator;

    #[test]
    fn base() {
        let mut originator = Editor::new();

        originator.set("first time input".to_string());
        println!("Step1: \ncurrent editor content is `{}`", originator.get_text());

        let memento = originator.save();

        originator.set("second time input.".to_string());
        println!("Step2: \ncurrent editor content is `{}`", originator.get_text());

        originator.restore(memento.as_ref());
        println!("Step3: \nrestore from snapshot, current editor content is `{}`", originator.get_text());
    }
}