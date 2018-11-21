use std::env;

pub fn get_path(path: &str) -> String {
    let here = {
        match env::current_exe() {
            Ok(p) => {
                match p.parent() {
                    Some(s) => { String::from(s.to_str().unwrap()) }//TODO: remove unwrap
                    None => {
                        panic!("Path not available {:?}", p)
                    }
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    };

    (here + "/" + path)
}