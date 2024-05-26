mod snake;

use piston_window::*;

fn main() {
    let title = "Hello Piston! (press any key to enter inner loop)";
    let mut window: PistonWindow = WindowSettings::new(title, [720, 720])
        .resizable(false)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], 
                [100.0, 0.0, 100.0, 100.0],
                c.transform, g)
        });
        
        
    }
}