use super::methods_interface::DataMiner;

pub struct Template<M: DataMiner> {
    miner: M,
}

impl<M: DataMiner> Template<M> {
    pub fn new(miner: M) -> Template<M> {
        Template{
            miner
        }
    }
    
    /// 模版方法，这里决定了接口中一系列方法的执行顺序。
    pub fn process(&self, file: &str) {
        self.miner.read_file(file);
        self.miner.analyze();
        self.miner.output_result();
    }
}