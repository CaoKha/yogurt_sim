# Swap Chain
- First, render pipeline <strong>does not draw directly on the texture that is currently displayed</strong>. A typical pipeline draws to an off-screen texture, which replaces the currently displayed one only once it is complete.
- Second, drawing takes a <strong>different time</strong> than the framerate required by your application, so the GPU may have to wait until the next frame is needed.
- Last, <strong>these off-screen textures are reused</strong> as much as possible. As soon as a new texture is presented, the previous one can be reused as a target for the next frame. This whole texture swapping mechanism is implemented by the <strong>Swap Chain</strong> object.
