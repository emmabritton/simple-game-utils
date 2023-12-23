[![Crates.io](https://img.shields.io/crates/v/simple-game-utils)](https://crates.io/crates/simple-game-utils "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/simple-game-utils)](https://docs.rs/simple-game-utils "Documentation")

# Game Utils

Simple game utilities 

## Usage

### Cargo

In your Cargo.toml file add

```
simple-game-utils = { version = "0.3.3", features = ["controller", "serde", "sound] }
```

### Code

#### Timing

This program runs for 1 second then exits. 
```rust
//track passage of time
let mut timing = Timing::new(240); //UPS
//triggers after specified time has passed
let mut timer = Timer::new(1.0);
loop {
    //automatically updates based on how much time has passed since the last call
    timing.update();
    //returns true if time has run out
    if timer.update(timing) {
        break;
    }   
}
```

#### Sound

> Requires `sound` feature

```rust
let mut engine = AudioEngine::new().unwrap();
let mut sound = engine.load_from_bytes(&some_sound_bytes, duration).unwrap();
sound.play();
loop {
    timing.update();
    sound.update(&timing);
} 
```

#### Controller

> Requires `controller` feature

```rust
// This will work whether or not there's a controller plugged in
let mut controller = GameController::new().expect("Unable to init controller lib");

loop {
    controller.update();
    if controller.direction.up {
        println!("DPad UP pressed");
    }
}
```

#### Preferences

> Requires `prefs` feature

```rust
struct Settings {
    user: String,
    theme: usize
}

let prefs: AppPrefs<Settings> = AppPrefs::new("com","example","readme", || Settings::default()).unwrap();
println!("{}", prefs.data.user);
prefs.data.user = String::new("New");
prefs.save();
```

## Features

### prefs

Simple struct storage

### controller

Very basic controller support

- no support for choosing controllers

### sound

Basic sound effects or music playback

### serde

Adds serialization for some structs