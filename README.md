
## BevyHop

A Skill-Based Movement Game, inspired by Counter Strike Bhop/Surf Community Servers.

Uses [bevy_fps_controller](https://github.com/qhdwight/bevy_fps_controller) for [Source Engine](https://en.wikipedia.org/wiki/Source_(game_engine)) inspired movement with [Air Strafing/Bunny Hopping](https://adrianb.io/2015/02/14/bunnyhop.html).

Air Strafing works by changing direction mid air by only using the mouse and strafe keys.


### Controls

- Move - WASD
- Jump - SPACE/MWheel Down (Can also hold space to keep jumping)
- Toggle Auto-Bhop - SHIFT+SPACE
- Reset to Checkpoint - R
- Reset Level - SHIFT+R
- Pause - ESC


### Known Issues

- Janky Surf/Speed Boost.
- UI Design (TBD).
- Getting stuck during respawn (fixable with scheduling most likely).
- Edge detection/Crouch issue. (https://github.com/qhdwight/bevy_fps_controller/pull/46)
- MouseWheel jump scroll event not kept around long enough to trigger proper jump sometimes.



