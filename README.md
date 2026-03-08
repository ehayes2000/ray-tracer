This project started as an implementaiton of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html), 
but I've taken so many detours that it's diverged significantly. 

If you're interested in running examples change the `Camera::render_mutli` call to reflect the 
number of cpu cores you have. This isn't a well-optimized ray tracer so running in parallel is important. 

I spent some time writing this first as a fragment shader, then (when I figured out I didn't care about realtime)
as a compute shader. The GPU implementation is buggy and far behind the CPU implementation. Enable the
feature flag if you want to run the example.

![Render](renders/knight_smooth_cornell.png)
![Render](renders/cornell.png)
