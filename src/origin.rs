use super::*;

pub fn sum() -> std::io::Result<()> {
    let path = Path::new("./test/sum.pdl");
    // let path = Path::new("./templates/sum.pdl");
    // let mut file: File = File::open(&path).expect("file not exists");
    let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Error reading file");

    // println!("{contents}");
    // let mut sum = 0;
    // for line in contents.lines() {
    //     let n = line.parse::<i32>().expect("parsing number error");
    //     sum += n;
    // }

    // println!("sum is {sum}");

    if let Ok(num_of_bytes) = File::open(&path)?.read_to_string(&mut contents) {
        println!("num_of_bytes:{num_of_bytes}");
        println!("{contents}");
        let mut sum = 0;
        for line in contents.lines() {
            let n = line.parse::<i32>().expect("parsing number error");
            sum += n;
        }
        println!("sum is {sum}");
    }

    println!("ok");
    Ok(())
}
