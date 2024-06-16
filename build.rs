use std::env;

fn main() {
    for (name, _value) in env::vars() {
        println!("{name}");

        if name.starts_with("DEP_FFMPEG_CHECK_") {
            println!(
                r#"cargo:rustc-check-cfg=cfg(feature, values("{}"))"#,
                name["DEP_FFMPEG_CHECK_".len()..name.len()].to_lowercase()
            );
        } else if name.starts_with("DEP_FFMPEG_") {
            println!(
                r#"cargo:rustc-cfg=feature="{}""#,
                name["DEP_FFMPEG_".len()..name.len()].to_lowercase()
            );
        }
    }
}
