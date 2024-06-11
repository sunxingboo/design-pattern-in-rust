mod singleton_lazy_static;
mod singleton_std;

#[cfg(test)]
mod tests {
    use crate::patterns::creational::singleton::singleton_lazy_static::SingletonLazyStatic;
    use crate::patterns::creational::singleton::singleton_std::SingletonStd;

    #[test]
    fn singleton_std() {
        let ins1 = unsafe { SingletonStd::new() };
        let ins2 = unsafe { SingletonStd::new() };

        let p1: String;
        let p2: String;
        
        {
            let s1 = ins1.lock().unwrap();
            p1 = format!("{:p}", &*s1);
        }

        {
            let s2 = ins2.lock().unwrap();
            p2 = format!("{:p}", &*s2);
        }

        assert_eq!(p1, p2);
    }

    #[test]
    fn singleton_lazy_static() {
        let ins1 = SingletonLazyStatic::new();
        let ins2 = SingletonLazyStatic::new();

        let p1: String;
        let p2: String;

        {
            let s1 = ins1.lock().unwrap();
            p1 = format!("{:p}", &*s1);
        }

        {
            let s2 = ins2.lock().unwrap();
            p2 = format!("{:p}", &*s2);
        }

        assert_eq!(p1, p2);
    }
}