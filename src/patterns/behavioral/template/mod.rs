mod methods_interface;
mod concrete_class;
mod template;

#[cfg(test)]
mod tests {
    use crate::patterns::behavioral::template::concrete_class::{CSVMiner, PDFMiner};
    use crate::patterns::behavioral::template::template::Template;

    #[test]
    fn base() {
        let temp = Template::new(PDFMiner::new());
        temp.process("file.pdf");

        let temp = Template::new(CSVMiner::new());
        temp.process("file.csv");
    }
}