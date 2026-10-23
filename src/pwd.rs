pub struct Pwd {

}

impl Pwd {
    pub fn process(&self) -> i32 {
        let curr_path = std::env::current_dir().unwrap();

        println!("{:?}", curr_path);

        0
    }
}
