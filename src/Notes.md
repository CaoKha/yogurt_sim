# Shader
- vs_main() entry for vertex shader, fs_main() entry for fragment shader
- `@builtin(position)` -> position in framebuffer space 
- framebuffer coordinates address the pixels in the framebuffer:
    * A coordinate that start from top-left (x to the right, y down) 
    * Related to `Render Passes` and `Rasterization`
- `clip_position` (analogous to GLSL `gl_Position`)
- clip coordinates is vec4
- `var` is mut but statically type
- `let` is not mut but dynamically type
- `location(0)` tells WGPU to store the return value vec4 in the first color target.



