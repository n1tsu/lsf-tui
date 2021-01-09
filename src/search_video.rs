use gstreamer::prelude::*;
use regex::Regex;

use std::io;

pub fn search_video(word: String) -> Result<(), io::Error> {
    let link = format!("https://dico.elix-lsf.fr/dictionnaire/{}", word);
    let body = reqwest::blocking::get(&link).unwrap().text().unwrap();
    let req = Regex::new("<video src=\"(https://www.elix-lsf.fr/.+?mp4)\"></video>").unwrap();

    let mut videos = Vec::new();
    for cap in req.captures_iter(&body) {
        videos.push(String::from(&cap[1]));
    };

    if videos.is_empty() {
        return Ok(())
    };

    gstreamer::init().unwrap();
    let playbin = gstreamer::ElementFactory::make("playbin", None).unwrap();
    playbin.set_property("uri", &videos[0]).unwrap();
    let bus = playbin.get_bus().unwrap();
    playbin
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the 'Playing' state");

    for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
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
