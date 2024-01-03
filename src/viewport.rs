use gl;

pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Viewport {
    pub fn for_window(x: i32, y: i32, w: i32, h: i32) -> Viewport {
        Viewport { x, y, w, h }
    }

    pub fn update_size(&mut self, w: i32, h: i32) {
        self.w = w;
        self.h = h;
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_used(&self) {
        unsafe {
            gl::Viewport(0, 0, self.w, self.h);
        }
    }
}
