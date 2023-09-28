use super::component::Component;

/// 组合节点（Composite）。
pub(crate) struct Directory {
    name: String,
    list: Vec<Box<dyn Component>>,
}

impl Component for Directory {
    fn open(&self) {
        println!("open path: {}", &self.name);
    }

    /// 打印简易的目录结构
    fn show(&self, deep: i32) {
        let mut prefix = String::new();
        for _ in 0..deep {
            prefix.push('-');
        }

        println!("{} {}", &prefix, &self.name);

        for i in &self.list {
            i.show(deep + 1);
        }
    }
}

impl Directory {
    pub(crate) fn new(name: String) -> Self {
        Directory {
            name,
            list: vec![],
        }
    }

    pub(crate) fn add(&mut self, c: Box<dyn Component>) {
        self.list.push(c);
    }
}