mod adapter;
mod plug;
mod outlet;

#[cfg(test)]
mod tests {
    use crate::patterns::structural::adapter::adapter::BritishStandardPlugAdapter;
    use crate::patterns::structural::adapter::plug::Plug;
    use super::plug::BritishStandardPlug;

    #[test]
    fn base() {
        // 英标插头
        let british_plug = BritishStandardPlug::new();
        british_plug.pin_shape();
        british_plug.voltage();

        // 使用适配器进行转换
        let adapter = BritishStandardPlugAdapter::new(
            Box::new(british_plug),
        );

        adapter.pin_shape();
        adapter.voltage();
    }
}