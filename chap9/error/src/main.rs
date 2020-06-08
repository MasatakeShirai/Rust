use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("crach and burn!");

    //let v = vec![1,2,3];
    //v[99];

    let f = File::open("hello.txt");
    let _f = match f{
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",e
                    )
                },
            }
        },
        Err(error) => {
            panic!("There was a ploblem oping the file: {:?}",error)
        },
    };

    //let _f = File::open("good.txt").unwrap();
    let _f = File::open("good.txt").expect("Failed to open good.txt");
}
