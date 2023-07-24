
#[derive(Default)]
pub struct FelinaeCat {
    name: String,
    age: i32,
    color: String,
}

impl FelinaeCat {
    pub fn new() -> Self {
        FelinaeCat::default()
    }

    // set name
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = format!("F-{}", name);

        self
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    // set age
    pub fn set_age(&mut self, age: i32) -> &mut Self {
        self.age = age + 100;

        self
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }

    // set color
    pub fn set_color(&mut self, color: String) -> &mut Self {
        self.color = format!("F-{}", color);

        self
    }

    pub fn get_color(&self) -> String {
        self.color.to_string()
    }

    pub fn say(&self) -> String {
        format!("I'm a felinae cat, my name is {}, my color is {}, my age is {}.", self.get_name(), self.get_color(), self.get_age())
    }
}