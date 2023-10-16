use super::methods_interface::DataMiner;

pub struct PDFMiner;

impl DataMiner for PDFMiner {
    fn read_file(&self, file: &str) {
        println!("PDF Miner: read contend from {}.", file);
    }

    fn analyze(&self) {
        println!("PDF Miner: analyze file content.");
    }

    fn output_result(&self) {
        println!("PDF Miner: output the result of this analyze.");
    }
}

impl PDFMiner {
    pub fn new() -> Self {
        PDFMiner
    }
}

pub struct CSVMiner;

impl DataMiner for CSVMiner {
    fn read_file(&self, file: &str) {
        println!("CSV Miner: read contend from {}.", file);
    }

    fn analyze(&self) {
        println!("CSV Miner: analyze file content.");
    }

    fn output_result(&self) {
        println!("CSV Miner: output the result of this analyze.");
    }
}

impl CSVMiner {
    pub fn new() -> Self {
        CSVMiner
    }
}
