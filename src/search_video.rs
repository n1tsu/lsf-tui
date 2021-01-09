use gstreamer::prelude::*;
use regex::Regex;

use std::io;


pub fn select_videos(urls: Vec<String>) -> Result<(), io::Error> {
    println!("{} videos found", urls.len());
    if urls.is_empty() {
        return Ok(())
    }

    for (i, uri) in urls.iter().enumerate() {
        println!("[{}] - {}", i, uri);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    let input_num: usize = match input.trim().parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
           println!("{}", e);
           return Ok(())
        }
    };
    if input_num >= urls.len() {
        println!("Input invalid");
        return Ok(())
    }

    play_video(&urls[input_num])
}

pub fn query_videos(word: &str) -> Vec<String> {
    let link = format!("https://dico.elix-lsf.fr/dictionnaire/{}", word);
    let body = reqwest::blocking::get(&link).unwrap().text().unwrap();

    let mut videos = Vec::new();

    let req = Regex::new("<video src=\"(https://www.elix-lsf.fr/.+?mp4)\"></video>").unwrap();
    for cap in req.captures_iter(&body) {
        videos.push(String::from(&cap[1]));
    };

    videos
}

pub fn play_video(uri: &str) -> Result<(), io::Error> {
    gstreamer::init().unwrap();
    let playbin = gstreamer::ElementFactory::make("playbin", None).unwrap();
    playbin.set_property("uri", &uri).unwrap();
    let bus = playbin.get_bus().unwrap();
    playbin
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the 'Playing' state");

    for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            /*
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            */
            MessageView::StateChanged(state_changed) =>
            // We are only interested in state-changed messages from playbin
            {
                if state_changed
                    .get_src()
                    .map(|s| s == playbin)
                    .unwrap_or(false)
                    && state_changed.get_current() == gstreamer::State::Playing
                {
                    // Generate a dot graph of the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                    let bin_ref = playbin.downcast_ref::<gstreamer::Bin>().unwrap();
                    bin_ref.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "PLAYING");
                }
            }

            _ => (),
        }
    }

    Ok(())
}
