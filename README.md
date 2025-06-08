
## BevyHop

A skill-based movement game, inspired by Counter Strike Bhop/Surf community servers.

![bevy_hop_gif](bevy_hop_gif.gif)

Uses [`bevy_fps_controller`](https://github.com/qhdwight/bevy_fps_controller) for Source Engine inspired movement with [`Air Strafing/Bunny Hopping`](https://adrianb.io/2015/02/14/bunnyhop.html).

Air Strafing works by changing direction mid air, using synced mouse and strafe keys (`A`,`D`), while not using `W` and `S`.


### Controls

- Move - `WASD`
- Jump - `SPACE`/`MWheel Down` (Can also hold space to keep jumping)
- Toggle Auto-Bhop - `SHIFT`+`SPACE`
- Reset to Checkpoint - `R`
- Reset Level - `SHIFT`+`R`
- Pause - `ESC`

For the best experience running natively is recommended.

### Dev controls/console

- Console - ``` ` ```

The console has a bunch of commands:

- `pause` - toggles pause/resume
- `debug` - toggles debug config (e.g. physics debug)
- `level {level}` - go to level
- `noclip` - fly/noclip

### Known Issues

- Mouse capture on itch.io bugging out
- Janky Surf/Speed Boost.
- HUD Design (TBD).
- [https://github.com/qhdwight/bevy_fps_controller/pull/46](Edge falloff detection/Crouch issue).
- MouseWheel jump scroll event not kept around long enough to trigger proper jump sometimes (might fix this one soon for hardcore bhoppers that need/want it).
- shadows cutting off after a certain distance.

### Early Release Notes

Some things aren't final i.e.
 - Level 3 Ending is a bit difficult.
 - Some Materials exports are messed up and need to be fixed in blender, especially end of lvl 3.
 - HUD/UI
 - maybe jump/walk sounds depending on how it turns out.
 - the `Run` time is basically the sum of the best segments. Failed attempts don't count. Could be improved/differentiated in the future.


### Things that were cut due to time constraints but might be added in the future

- Multiplayer
- Multiple types of boosts
- dynamic and more particle/audio fx
- Highscores


### Credits

- Color palette: [https://lospec.com/palette-list/resurr](Ressurect 64 by Kerrie Lake)
- HRDI Turnberg Sky 3 - [https://hdrmaps.com/turnberg-sky-3/](HDRMAPS.com Royalty-Free)
- HDRI Turnberg Sky 4 -  [https://hdrmaps.com/turnberg-sky-4/](HDRMAPS.com Royalty-Free)
- HDRI Turnberg Sky 5 - [https://hdrmaps.com/turnberg-sky-5/](HDRMAPS.com Royalty-Free )
- Ocean sound - [https://pixabay.com/sound-effects/gentle-ocean-waves-fizzing-bubbles-64980/](Gentle ocean waves fizzing bubbles by jackmichaelking (Freesound)) Pixabay Content License
- Dive sound - [https://uppbeat.io/sfx/underwater-dive-impact/8179/24475](Underwater dive impact by Epic Stock Media)
