
use design_pattern::structural_pattern::adapter_pattern::{MediaPlayer, AudioPlayer};

fn main() {
   let autio_player:  AudioPlayer = AudioPlayer{};

    autio_player.play("mp3".to_owned(), "beyond the horizon.mp3".to_owned());
    autio_player.play("mp4".to_owned(), "alone.mp4".to_owned());
    autio_player.play("vlc".to_owned(), "far far away.vlc".to_owned());
    autio_player.play("avi".to_owned(), "mind me.avi".to_owned());


}
