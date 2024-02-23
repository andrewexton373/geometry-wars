pub fn progress_string(progress: f32) -> String {
    let progress_bar_len = 10;

    (0..progress_bar_len)
        .map(|i| {
            let percent = i as f32 / progress_bar_len as f32;
            if percent < progress {
                '◼'
            } else {
                '◻'
            }
        })
        .collect::<String>()
}
