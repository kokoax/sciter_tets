extern crate sciter;

fn main() {
    let mut frame = sciter::Window::new();

    frame.load_file("index.html");

    frame.run_app();
}
