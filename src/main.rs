mod lyapunov;

use lyapunov::Lyapunov;

fn main() {
    let l = Lyapunov::new(String::from("ab"));
    l.generate_image()
}
