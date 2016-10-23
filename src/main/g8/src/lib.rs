
pub mod build;

pub fn hello(){
    println!("Hello world :)");
}


#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
