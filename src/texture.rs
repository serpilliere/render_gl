use anyhow::Result;
use gl;
use std::os::raw;

pub struct Texture {
    obj: gl::types::GLuint,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.obj) };
    }
}

impl Texture {
    pub fn new_blank() -> Result<Texture> {
        let mut obj: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut obj);
        }

        let texture = Texture { obj };

        Ok(texture)
    }

    pub fn load_texture_data(
        format: (gl::types::GLuint, gl::types::GLuint),
        texture: &Texture,
        data: &(u32, u32, &[u8]),
    ) -> Result<()> {
        let obj = texture.obj;
        let (img_w, img_h, data) = (data.0, data.1, data.2);
        //println!("texture id: {}", obj);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, obj);
        }
        //println!("texture id: {}", obj);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, obj);
        }

        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format.0 as gl::types::GLint,
                img_w as i32,
                img_h as i32,
                0,
                format.1,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const raw::c_void,
            );
        }

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.obj);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind_at(&self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
        }
        self.bind();
    }
}
