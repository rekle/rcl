pub fn process(long_output: &bool)
{
    if *long_output  {
        println!("ls -1")
    }
    else {
        println!("ls")
    }
}