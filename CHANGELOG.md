# Changelog

### Version 0.5.3
- Add getter methods for Tileset

### Version 0.5.2
### BREAKING
- Replace `Tilemap::onscreen_px_for_tile` with `Tilemap::orig_px_for_tile`
- Add `Tilemap::px_for_tile`

### Version 0.5.1
- Fix bug with controllers

### Version 0.5.0 (pulled)
- Add `Tilemap`
- Remove exact deps versions
- Update deps

### Version 0.4.2
- Add `derive` `clone` and `debug` to AppPrefs
- Add `reload()` to AppPrefs

### Version 0.4.1
- Fix serde feature
- Enable serde feature by default

### Version 0.4.0
- Add xinput feature
- controller feature no longer enabled by default 

### Version 0.3.4
- Add derive to controller type
- Fix serde derive for controller enums

### Version 0.3.3
- Add controller type and method

### Version 0.3.2
- Add preferences

### Version 0.3.1
- Add `Timer::delay()`

### Version 0.3.0
- Add `Timing` and `Timer`
- Add `SoundEffect`

### Version 0.2.0
- Add `any_connected(): bool`
- Remove `Gilrs` from constructor parameters

### Version 0.1.0
- Initial release