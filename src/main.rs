use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid command line argument count, only input image file");
    }

    let image_path = args[0].as_str();

}
