
## BevyHop

A skill-based movement game, inspired by Counter Strike Bhop/Surf community servers.

![bevy_hop_gif](bevy_hop_gif.gif)

Uses [`bevy_fps_controller`][9] for Source Engine inspired movement with [`Air Strafing/Bunny Hopping`][8]
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

- `pause` - toggle pause/resume
- `debug` - toggle debug config (e.g. physics debug)
- `level {level}` - go to level
- `noclip` - fly/noclip
- `fps` - toggle fps counter

### Known Issues

- Mouse capture on itch.io bugging out
- Janky Surf/Speed Boost.
- HUD Design (TBD).
- [Edge falloff detection/Crouch issue][7].
- MouseWheel jump scroll event not kept around long enough to trigger proper jump sometimes (might fix this one soon for hardcore bhoppers that need/want it).
- shadows cutting off after a certain distance.

### Early Release Notes

Some things aren't final i.e.
 - maybe jump/walk sounds depending on how it turns out.
 - the `Run` time is basically the sum of the best segments. Failed attempts don't count. Could be improved/differentiated in the future.


### Things that were cut due to time constraints but might be added in the future

- Multiplayer
- Multiple types of boosts
- dynamic and more particle/sound fx
- Highscores


### Credits

- Code hosted on [Github][10]
- Color palette: [Ressurect 64 by Kerrie Lake][1]
- HRDI Turnberg Sky 3 - [HDRMAPS.com Royalty-Free][2]
- HDRI Turnberg Sky 4 - [HDRMAPS.com Royalty-Free][3]
- HDRI Turnberg Sky 5 - [HDRMAPS.com Royalty-Free][4]
- Ocean sound - [Gentle ocean waves fizzing bubbles by jackmichaelking (Freesound) Pixabay Content License][5]
- Dive sound - [Underwater dive impact by Epic Stock Media][6]
- Default Font - [Fira Mono by Carrois Apostrophe, The Mozilla Corporation and Telefonica S.A.  SIL Open Font License, Version 1.1][11]
- Header Font - [Jua by Woowahan Brothers, The Jua Project Authors -SIL Open Font License, Version 1.1]


[1]: https://lospec.com/palette-list/resurrect-64
[2]:https://hdrmaps.com/turnberg-sky-3/
[3]:https://hdrmaps.com/turnberg-sky-4/
[4]:https://hdrmaps.com/turnberg-sky-5/
[5]:https://pixabay.com/sound-effects/gentle-ocean-waves-fizzing-bubbles-64980/
[6]:https://uppbeat.io/sfx/underwater-dive-impact/8179/24475
[7]:https://github.com/qhdwight/bevy_fps_controller/pull/46#issuecomment-2889270436
[8]:https://adrianb.io/2015/02/14/bunnyhop.html
[9]:https://github.com/qhdwight/bevy_fps_controller
[10]:https://github.com/NicoZweifel/BevyHop
[11]:https://fonts.google.com/specimen/Fira+Mono
[12]:https://fonts.google.com/specimen/Jua
