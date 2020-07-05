mod vec;

fn main() {
    let n_rows = 255;
    let n_cols = 255;
    println!("P3\n{} {}\n255", n_cols, n_rows);
    for y in 0..n_rows {
        for x in 0..n_cols {
            let pix = vec::Color::new(
                x as f32 / n_cols as f32,
                y as f32 / n_rows as f32,
                0.25
            );
            pix.print_color();
        }
    }
}
