
#[derive(Default)]
pub struct BlueWhiteCat {
    name: String,
    age: i32,
    color: String,
}

impl BlueWhiteCat {
    pub fn new() -> Self {
        BlueWhiteCat::default()
    }

    // name
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = format!("B-{}", name);

        self
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    // age
    pub fn set_age(&mut self, age: i32) -> &mut Self {
        self.age = age + 99;

        self
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }

    // color
    pub fn set_color(&mut self, color: String) -> &mut Self {
        self.color = format!("B-{}", color);

        self
    }

    pub fn get_color(&self) -> String {
        self.color.to_string()
    }

    pub fn say(&self) -> String {
        format!("I'm a blue white cat, my name is {}, my color is {}, my age is {}.", self.get_name(), self.get_color(), self.get_age())
    }
}