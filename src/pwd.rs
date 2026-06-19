pub fn process() {
    let curr_path = std::env::current_dir().unwrap();

    println!("{:?}", curr_path);
}