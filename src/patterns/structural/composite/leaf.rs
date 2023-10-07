use super::component::Component;

/// 叶子节点（Leaf）。是整个系统结构真正最小粒度的组成元件。
pub struct File {
    name: String,
    content: String,
}

impl Component for File {
    fn open(&self) {
        println!("open file: {}", &self.name);
    }

    // 展示文件名称与内容
    fn show(&self, deep: i32) {
        let mut prefix = String::new();
        for i in 0..deep {
            prefix.push('-');
        }

        print!("{} {}", &prefix, &self.name);
        println!("{}", &self.content);
    }
}

impl File {
    pub fn new(name: String) -> Self {
        File {
            name,
            content: String::new(),
        }
    }
}