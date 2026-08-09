use ffmpeg_the_third as ffmpeg;

use std::env;

#[cfg(feature = "ffmpeg_9_0")]
fn print_supported<I>(label: &str, supported: ffmpeg::codec::config::Supported<I>)
where
    I: Iterator,
    I::Item: std::fmt::Debug,
{
    match supported {
        ffmpeg::codec::config::Supported::All => println!("\t {label}: any"),
        ffmpeg::codec::config::Supported::Specific(values) => {
            println!("\t {label}: {:?}", values.collect::<Vec<_>>())
        }
    }
}

fn main() {
    ffmpeg::init().unwrap();

    for arg in env::args().skip(1) {
        if let Some(codec) = ffmpeg::decoder::find_by_name(&arg) {
            println!("type: decoder");
            println!("\t id: {:?}", codec.id());
            println!("\t name: {}", codec.name());
            println!("\t description: {}", codec.description());
            println!("\t medium: {:?}", codec.medium());
            println!("\t capabilities: {:?}", codec.capabilities());

            if let Some(profiles) = codec.profiles() {
                println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
            } else {
                println!("\t profiles: none");
            }

            if let Some(video) = codec.video() {
                #[cfg(not(feature = "ffmpeg_9_0"))]
                {
                    if let Some(rates) = video.rates() {
                        println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                    } else {
                        println!("\t rates: any");
                    }

                    if let Some(formats) = video.formats() {
                        println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                    } else {
                        println!("\t formats: any");
                    }
                }

                #[cfg(feature = "ffmpeg_9_0")]
                {
                    print_supported("rates", video.supported_rates());
                    print_supported("formats", video.supported_formats());
                }
            }

            if let Some(audio) = codec.audio() {
                #[cfg(not(feature = "ffmpeg_9_0"))]
                {
                    if let Some(rates) = audio.rates() {
                        println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                    } else {
                        println!("\t rates: any");
                    }

                    if let Some(formats) = audio.formats() {
                        println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                    } else {
                        println!("\t formats: any");
                    }

                    if let Some(layouts) = audio.ch_layouts() {
                        println!("\t channel_layouts: {:?}", layouts.collect::<Vec<_>>());
                    } else {
                        println!("\t channel_layouts: any");
                    }
                }

                #[cfg(feature = "ffmpeg_9_0")]
                {
                    print_supported("rates", audio.supported_rates());
                    print_supported("formats", audio.supported_formats());
                    print_supported("channel_layouts", audio.supported_layouts());
                }
            }

            println!("\t max_lowres: {:?}", codec.max_lowres());
        }

        if let Some(codec) = ffmpeg::encoder::find_by_name(&arg) {
            println!();
            println!("type: encoder");
            println!("\t id: {:?}", codec.id());
            println!("\t name: {}", codec.name());
            println!("\t description: {}", codec.description());
            println!("\t medium: {:?}", codec.medium());
            println!("\t capabilities: {:?}", codec.capabilities());

            if let Some(profiles) = codec.profiles() {
                println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
            }

            if let Some(video) = codec.video() {
                #[cfg(not(feature = "ffmpeg_9_0"))]
                {
                    if let Some(rates) = video.rates() {
                        println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                    } else {
                        println!("\t rates: any");
                    }

                    if let Some(formats) = video.formats() {
                        println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                    } else {
                        println!("\t formats: any");
                    }
                }

                #[cfg(feature = "ffmpeg_9_0")]
                {
                    print_supported("rates", video.supported_rates());
                    print_supported("formats", video.supported_formats());
                }
            }

            if let Some(audio) = codec.audio() {
                #[cfg(not(feature = "ffmpeg_9_0"))]
                {
                    if let Some(rates) = audio.rates() {
                        println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                    } else {
                        println!("\t rates: any");
                    }

                    if let Some(formats) = audio.formats() {
                        println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                    } else {
                        println!("\t formats: any");
                    }

                    if let Some(layouts) = audio.ch_layouts() {
                        println!("\t channel_layouts: {:?}", layouts.collect::<Vec<_>>());
                    } else {
                        println!("\t channel_layouts: any");
                    }
                }

                #[cfg(feature = "ffmpeg_9_0")]
                {
                    print_supported("rates", audio.supported_rates());
                    print_supported("formats", audio.supported_formats());
                    print_supported("channel_layouts", audio.supported_layouts());
                }
            }

            println!("\t max_lowres: {:?}", codec.max_lowres());
        }
    }
}
