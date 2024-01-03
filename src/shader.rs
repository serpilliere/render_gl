use anyhow::{Context, Result};
use gl;
use nalgebra as na;
use std;
use std::ffi::{CStr, CString};

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(anyhow!("shader error: {}", error.to_string_lossy()));
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn from_frag_vert_sources(source_vert: &str, source_frag: &str) -> Result<Program> {
        let source_vert = CString::new(source_vert).context("Error in converting vert shader")?;
        let source_frag = CString::new(source_frag).context("Error in converting frag shader")?;

        let shader_vert = Shader::from_vert_source(source_vert.as_c_str())
            .context("Error in loading vert shader")?;
        let shader_frag = Shader::from_frag_source(source_frag.as_c_str())
            .context("Error in loading frag shader")?;
        let shaders = vec![shader_vert, shader_frag];
        Program::from_shaders(&shaders[..])
            .map_err(|message| anyhow!("Error from_shaders: {:?}", message))
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<i32> {
        let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

        let location = unsafe {
            gl::GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8)
        };

        if location == -1 {
            return None;
        }

        Some(location)
    }

    pub fn set_uniform_matrix_4fv(&self, location: i32, value: &na::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }

    pub fn set_uniform_3f(&self, location: i32, value: &na::Vector3<f32>) {
        unsafe {
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn set_uniform_1i(&self, location: i32, index: i32) {
        unsafe {
            gl::Uniform1i(location, index);
        }
    }

    pub fn set_uniform_1f(&self, location: i32, value: f32) {
        unsafe {
            gl::Uniform1f(location, value);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader> {
        Shader::from_source(source, gl::VERTEX_SHADER)
            .map_err(|err| anyhow!("Error in shader vert from source {}", err))
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
            .map_err(|err| anyhow!("Error in shader frag from source {}", err))
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(anyhow!("shader error: {}", error.to_string_lossy()));
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
