use audio_engine::SineWave;
use simple_game_utils::prelude::*;

fn main() {
    let timing = Timing::new(60);
    let mut timer = Timer::new_once(2.0);
    assert!(!timer.update(&timing));

    let engine = AudioEngine::new().unwrap();
    let mut sound = engine.new_sound(SineWave::new(48000, 100.0)).unwrap();

    sound.play();
    timer.update(&timing);

    let mut controller = GameController::new().unwrap();
    controller.update();
    assert!(!controller.menu.start);
}
