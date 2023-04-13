/*
    ❯ rustc emoji.rs && ./emoji
    🐣
 */


fn main() {
    // from https://symbl.cc/en/1F423/ > Encoding > dec (bytes)
    let hatching_chick_bytes = vec![240, 159, 144, 163];
    let hatching_chick = String::from_utf8(hatching_chick_bytes).unwrap();
    println!("{hatching_chick}");
}
