use super::plug::Plug;

/// 适配器模式主要应用于需要使结构不同的两个接口之间可以适配，即让原本由于接口不兼容而不能一起工作的两个类可以协同工作。
/// 如通过公牛转换头，将港版插头与大陆插座适配。这里任务转换头是港版插头的适配器（也就是适配器是为了适配使用者）。

/// 英标插头适配器。
pub(crate) struct BritishStandardPlugAdapter {
    plug: Box<dyn Plug>
}

impl BritishStandardPlugAdapter {
    pub(crate) fn new(plug: Box<dyn Plug>) -> Self {
        BritishStandardPlugAdapter {
            plug,
        }
    }
}

impl Plug for BritishStandardPlugAdapter {
    fn pin_shape(&self) {
        self.plug.pin_shape();
        println!("converted: national standard shape.");
    }

    fn voltage(&self) {
        self.plug.voltage();
        println!("converted: national standard 220V.")
    }
}