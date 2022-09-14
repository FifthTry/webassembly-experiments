wit_bindgen_guest_rust::export!("../wits/guest.wit");

wit_bindgen_guest_rust::import!("../wits/host.wit");

struct Guest;

impl guest::Guest for Guest {
    fn run(a: String) -> String {
        use host;
        let resp = host::http(host::Httprequest {
            path: "https://www.fishwatch.gov/api/species/atlantic-sharpnose-shark",
            method: "GET",
        });
        format!("{}", resp.data)
    }
}
