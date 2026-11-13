pub fn generate_ppm(width: usize, height: usize, data: &[u8], skip_fourth: bool) -> Vec<u8> {
    let mut vec = Vec::new();

    vec.extend(format!("P6\n{} {}\n255\n", width, height).as_bytes());

    for (i, v) in data.iter().enumerate() {
        if i % 4 != 3 || !skip_fourth {
            vec.push(*v);
        }
    }

    vec
}
