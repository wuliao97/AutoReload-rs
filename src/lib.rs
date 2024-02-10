pub mod ar;

// use ar

#[cfg(test)]
mod tests {
    use crate::ar::{ARTrait, AR, Error};

    struct Person {
        name: String
    }
    impl Person {
        fn self_introduce(&self) {
            println!("My name is ...{}", self.name);
        }
    }

    #[async_trait::async_trait]
    impl ARTrait for Person {
        async fn execute(&self) -> Result<(), Error> {
            self.self_introduce();
            Ok(())
        } 
    }

    #[tokio::test]
    async fn it_works() {
        let p = Person { name: String::from("Ennui") };
        let ar = AR::new(p, 2);
        println!("Start");
        ar.start().await.ok();
    }
}
