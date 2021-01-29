// https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };    
}

fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}


pub fn lazy_eval_sample()
{
    let access = PRIVILEGES.get("james");
    println!("James: {:?}", access);
    show_access("Jim");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lazy_eval() {
        let access = super::PRIVILEGES.get(&"Jim");
        assert_eq!(Some(<vec!["user"]>), access);
        let access = super::PRIVILEGES.get(&"James");
        assert_eq!(Some(<vec!["user"]>), access);
    }
}