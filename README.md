
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


### Known Issues

- Janky Surf/Speed Boost.
- HUD Design (TBD).
- Edge falloff detection/Crouch issue. (https://github.com/qhdwight/bevy_fps_controller/pull/46)
- MouseWheel jump scroll event not kept around long enough to trigger proper jump sometimes (might fix this one soon for hardcore bhoppers that need/want it).



