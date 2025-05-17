const WIDTH: usize = 21;
const HEIGHT: usize = 21;

pub fn draw_envelope() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0  y == HEIGHT - 1  x == 0  x == WIDTH - 1  x == y || x + y == WIDTH - 1 {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}