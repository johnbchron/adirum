# adirum

`adirum` is a little space station game I'm working on, built with `bevy` and `ratatui`.

For those interested in the renderer, here is a mini breakdown of how it works.


- Explanation of some pieces:
  - The `ratatui` context is handled by the `bevy_ratatui` plugin. Inputs are funky.
  - The `RenderBuffer` resource
    - A `bevy` resource that holds the state for the `RenderedWidget`.
    - This is what all `bevy`-side items interact with to get rendered.
  - The `RenderedWidget` widget
    - A stateful widget that holds a `ratatui` buffer and a "last size". The buffer gets splatted to the main buffer and is cropped or centered to make up for the size difference.
  - The `prepare_for_frame` system
    - Runs in `PreUpdate`. Resizes the `RenderBuffer` and updates it with the most recent `CameraMatrix`. 
  - The `CameraMatrix` component
    - A `bevy` component that holds the camera's current world-to-local/view matrix (`view`) and local/view-to-projection matrix (`proj`)
  - The `update_camera_matrices` system
    - Runs in `PostUpdate` after cameras have been mutated, and updates their `CameraMatrix` component.
  - The `Camera` component
    - A `bevy` component that declares the entity a camera, and holds some basic config.
  - The `Shape` enum
    - An enum of all shapes
  - The `DrawnShape` trait
    - Allows shapes to be drawn to a `ShapeBuffer`.
  - The `ShapeBuffer` struct
    - Similar to a `ratatui` buffer but holds depth information so that many shape buffers can be combined and depth-sorted.
    - Combining shape buffers and splatting them to the `RenderBuffer` is not implemented yet, so I'm doing it somewhat manually by converting a single shape buffer to a `ratatui` buffer and then splatting that.
  - The `dummy_render` system
    - Runs in the `Render` schedule and renders a spinning cube

Lines are my single shape primitive right now.
All shapes have to have access to the camera projection because though they're specified in world-space, they have to know about how they'll show up in canvas-space (`ratatui` coordinates) to know how to draw themselves.
This is why the renderer is immediate in the way it is.

For example, a line can figure out where to draw its points in world-space easily, but it has to know its start and end coordinates in canvas-space to know how many points it needs to draw. Over-draw/under-draw kills ASCII-art. It also measures the perceived angle of each point's neighbors to figure out what character to draw. That logic happens in the `thin_neighbor` module.
