fn main() {
    loop {
        println!("Loop 1");

        loop {
            println!("Loop 2");
        }
    }
}

async fn foo() {
    println!("Hey, I'm Foo");
}

async fn bar() {
    println!("Hey, I'm Bar");
}
