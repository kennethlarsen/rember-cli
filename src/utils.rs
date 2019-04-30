use std::fs::File;
use std::io::{self, Read, Write};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

pub fn update_values_in_files(word_to_replace: &str, new_word: &str, path: &str) -> Result<(), io::Error> {
    let mut src = File::open(path)?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    drop(src);  // Close the file early

    let new_data = data.replace(word_to_replace, new_word);
    let mut dst = File::create(path)?;
    dst.write(new_data.as_bytes())?;

    Ok(())
}

pub fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };

    bar.set_message(msg);
    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] eta: {eta}")
                .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}
