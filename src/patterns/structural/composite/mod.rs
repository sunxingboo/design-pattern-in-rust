mod leaf;
mod component;
mod composite;

#[cfg(test)]
mod tests {
    use crate::patterns::structural::composite::component::Component;
    use crate::patterns::structural::composite::composite::Directory;
    use crate::patterns::structural::composite::leaf::File;

    #[test]
    fn base() {
        let mut root = Directory::new("root".to_string());

        let mut l1_d1 = Directory::new("l1_d1".to_string());
        let mut l1_d2 = Directory::new("l1_d2".to_string());
        let l1_f1 = File::new("l1_f1".to_string());
        let l1_f2 = File::new("l1_f2".to_string());

        let mut l2_d1 = Directory::new("l2_d1".to_string());
        let l2_f1 = File::new("l2_f1".to_string());

        let l3_d1 = Directory::new("l3_d1".to_string());
        let l3_f1 = File::new("l3_f1".to_string());
        let l3_f2 = File::new("l3_f2".to_string());

        l2_d1.add(Box::new(l3_d1));
        l2_d1.add(Box::new(l3_f1));
        l2_d1.add(Box::new(l3_f2));

        l1_d1.add(Box::new(l2_f1));
        l1_d2.add(Box::new(l2_d1));

        root.add(Box::new(l1_d1));
        root.add(Box::new(l1_d2));
        root.add(Box::new(l1_f1));
        root.add(Box::new(l1_f2));

        root.show(0);
    }
}