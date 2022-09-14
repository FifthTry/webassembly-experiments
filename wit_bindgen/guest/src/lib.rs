wit_bindgen_guest_rust::export!("../wits/say.wit");

struct Say;

impl say::Say for Say {
    fn hello(name: String) -> String {
        let rc = format!("hello {}", name);
        rc
    }
}

