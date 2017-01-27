mod struct_mod {
    pub struct Foo {
        private: i32,
        pub public: i32,
    }
    impl Foo {
        pub fn new() -> Self {
            Foo {
                private: 42,
                public: 41,
            }
        }

        pub fn modify(&mut self) {
            self.private -= 2;
            self.private -= 2;
        }
    }
}

use struct_mod::*;

fn main() {
    let mut bar = Foo::new();
    println!("{}", &bar.public);
    bar.modify();
    println!("{}", &bar.private);
}
