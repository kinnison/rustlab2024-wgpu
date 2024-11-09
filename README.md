# RustLab 2024 wgpu workshop

Chapters:

1. Basics of wgpu
2. Tracing rays (with compute shaders)
3. Bringing it to the web (with WebAssembly)

Checkout the respective branch for the chapter you are currently working on (ch1, ch2 or ch3).

## Chapter 1

Tasks:

1. Try running the application with `cargo run` to ensure your setup is working.
2. Implement methods in `src/application.rs`
    - `Application::new`
    - `Application::render`
    - `Application::resize`
    - Every method has an enumerated list of tasks required to finish the implementation.
    - After every method, try running your application to check for validation errors.
3. Implement the shader in `src/application.wgsl`
    - The shader lists the required tasks that are needed to complete the implementation.
    - Try to get creative and find out what else you can draw using just the fragment shader.
