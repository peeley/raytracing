mod vec;

fn main() {
    let n_rows = 255;
    let n_cols = 255;
    let max_color = 255.0;
    println!("P3\n{} {}\n255", n_cols, n_rows);
    for y in 0..n_rows {
        for x in 0..n_cols {
            let red: f32 = x as f32 / n_cols as f32;
            let green: f32 = y as f32 / n_rows as f32;
            let blue: f32 = max_color as f32 / 4.0;
            let _vec = vec::Vec3::new(1.0, 2.0, 3.0);
            println!(
                "{} {} {}",
                (red * max_color) as u8,
                (green * max_color) as u8,
                blue as u8
            );
        }
    }
}
