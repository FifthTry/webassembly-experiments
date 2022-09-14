wit_bindgen_guest_rust::export!("../wits/say.wit");

struct Say;

impl say::Say for Say {
    fn hello(name: String) -> String {
        let rc = if name != "Michael" {
            format!("I don't know you! {}", name)
        } else {
            format!("Hello good sir {}", name)
        };

        rc
    }
}
