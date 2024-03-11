pub mod read_package_list;

fn main() {
    let args: Vec<String> = (&std::env::args().collect::<Vec<String>>())[1..].to_vec();
    // dbg!(&args);
    if &args[0] == "i" || &args[0] == "install" {
        println!("hi you want to install")
    }
}