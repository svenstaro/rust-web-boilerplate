extern crate rust_web_boilerplate;

fn main() {
    let (rocket, _) = rust_web_boilerplate::rocket_factory();
    rocket.launch();
}
