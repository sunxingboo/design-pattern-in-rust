pub trait DataMiner {
    fn read_file(&self, _: &str);
    fn analyze(&self);
    fn output_result(&self);
}