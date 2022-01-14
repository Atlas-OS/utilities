fn main() {
    // get first argument as string
    // example "exe"
    let args: Vec<String> = std::env::args().collect();
    // check if argument is empty
    if args.len() == 2 {
        // launch prompt
        let res = rfd::FileDialog::new()
            // add file extension filter
            .add_filter(&args[1], &[&args[1]])
            .pick_file();
        // check if file prompt was cancelled
        if res == None {
            println!("cancelled by user");
        } else{
            println!("{}", res.unwrap().to_str().unwrap().replace("\\", "/"));
        }
    } else {
        let res = rfd::FileDialog::new()
            .add_filter("All Files", &["*.*"])
            .pick_file();
        if res == None {
            println!("cancelled by user");
        } else {
            println!("{}", res.unwrap().to_str().unwrap().replace("\\", "/"));
        }
    }
}