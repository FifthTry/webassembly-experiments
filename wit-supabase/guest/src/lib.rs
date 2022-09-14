wit_bindgen_guest_rust::export!("../wits/guest.wit");

struct Guest;

impl guest::Guest for Guest {
    fn run(a: String) -> String {
        format!("Awesome!!! {}", a)
    }
}
