## Uniform buffers and a 3d camera

### A perspective camera
#### Homogenous Matrix Revision:
In the context of a 4x4 transformation matrix, the fourth column vector (also known as the fourth column) typically represents the translation component of the transformation.
The first three columns of the matrix handle rotation, scaling, or other transformation operations, while the fourth column handles translation. The fourth column vector contains the translation values along the x, y, and z axes.
For example, consider the following transformation matrix:

```css
[ v11  v21  v31  tx ]
[ v12  v22  v32  ty ]
[ v13  v23  v33  tz ]
[  0    0    0    1 ]
```
The fourth column [tx, ty, tz] represents the translation component of the transformation. These values determine the displacement or position of the transformed coordinate system relative to the original coordinate system.
When applying the transformation matrix to a point or vector, the translation component is added to the resulting transformed coordinates. It allows you to shift or move the object in the transformed coordinate system.
Keep in mind that different conventions and frameworks may use different layouts or interpretations of transformation matrices. Always refer to the specific documentation or conventions of the system or library you are working with to ensure accurate interpretation and usage of the transformation matrix.
#### camera
- The `view` matrix move the world to be at the position and rotationof the camera. It's essentially an inverse of whatever the transform matrix of the camera would be.
- The `proj` matrix warps th scene to give the effect of depth.
- The coordinate system in Wgpu is based on DirectX, and Metal's coordinate systems. That means that in `normalized device coordinates` 

