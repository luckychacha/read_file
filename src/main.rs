use read_file;

fn main() {
    match read_file::sum() {
        Ok(_) => {
            println!("Ok");
        }
        Err(_) => todo!(),
    }
}
