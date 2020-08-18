pub struct Renderer {
    pub cols: i32,
    pub rows: i32,
    pub scale: i32,
}

impl Renderer {
    pub fn new(scale: i32) -> Renderer {
        Renderer {
            cols: 64,
            rows: 32,
            scale: scale,
        }
    }

    pub fn draw() {
        // let window = video_subsystem.window("Example", 800, 600).build().unwrap();
    }
}