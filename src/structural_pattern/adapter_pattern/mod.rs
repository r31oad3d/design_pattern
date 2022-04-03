pub trait MediaPlayer {
    fn play(&self, audio_type: String, file_name: String);
}

pub trait AdvancedMediaPlayer {
    fn play_vlc(&self, file_name: String);
    fn play_mp4(&self, file_name: String);
}

pub struct VlcPlayer;

impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: String) {
        println!("Playing vlc file. Name: {}", file_name);
    }

    fn play_mp4(&self, _file_name: String) {
        unimplemented!()
    }
}

pub struct Mp4Player;

impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, _file_name: String) {
        unimplemented!()
    }

    fn play_mp4(&self, file_name: String) {
        println!("Playing mp4 file. Name: {}", file_name);
    }
}

pub struct MediaAdapter {
    advance_media_player: Box<dyn AdvancedMediaPlayer>,
}
impl MediaAdapter {
    pub fn new(audio_type: String) -> Self {
        if audio_type.contains("vlc") {
            MediaAdapter {
                advance_media_player: Box::new(VlcPlayer),
            }
        } else if audio_type.contains("mp4") {
            MediaAdapter {
                advance_media_player: Box::new(Mp4Player),
            }
        } else {
            unimplemented!()
        }
    }
}

impl MediaPlayer for MediaAdapter {
    fn play(&self, audio_type: String, file_name: String) {
        if audio_type.contains("vlc") {
            self.advance_media_player.play_vlc(file_name);
        } else if audio_type.contains("mp4") {
            self.advance_media_player.play_mp4(file_name);
        } else {
            unimplemented!()
        }
    }
}

pub struct AudioPlayer {}
impl MediaPlayer for AudioPlayer {
    fn play(&self, audio_type: String, file_name: String) {
        if audio_type.contains("mp3") {
            println!("Playing mp3 file. Name is {}", file_name);
        } else if audio_type.contains("vlc") || audio_type.contains("mp4") {
            MediaAdapter::new(audio_type.clone()).play(audio_type, file_name);
        } else {
            println!("Invalid media. {} format not supported", audio_type);
        }
    }
}
