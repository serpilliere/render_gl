use gl;

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub struct Buffer<B>
where
    B: BufferType,
{
    pub vbo: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<B>,
}

impl<B> Default for Buffer<B>
where
    B: BufferType,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<B> Buffer<B>
where
    B: BufferType,
{
    /// # Safety
    /// Only alloc here: Everything is ok?
    pub fn new() -> Buffer<B> {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Buffer {
            vbo,
            _marker: ::std::marker::PhantomData,
        }
    }

    /// # Safety
    /// self.vbo must be valid opengl object
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    /// # Safety
    /// Everything is ok?
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    /// # Safety
    /// data.ptr mut be ok
    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,                                       // target
                std::mem::size_of_val(data) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid,            // pointer to data
                gl::STATIC_DRAW,                                      // usage
            );
        }
    }

    /// # Safety
    /// data.ptr mut be ok
    pub fn dynamic_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,                                       // target
                std::mem::size_of_val(data) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid,            // pointer to data
                gl::DYNAMIC_DRAW,                                     // usage
            );
        }
    }

    /// # Safety
    /// Only alloc here: Everything is ok?
    pub fn dynamic_draw_data_null<T>(&self, size: usize) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,                                               // target
                (size * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                ::std::ptr::null() as *const gl::types::GLvoid,               // pointer to data
                gl::DYNAMIC_DRAW,                                             // usage
            );
        }
    }

    /// # Safety
    /// Only alloc here: Everything is ok?
    pub unsafe fn map_buffer_range_write_invalidate<'r, T>(
        &self,
        offset: usize,
        size: usize,
    ) -> Option<MappedBuffer<'r, B, T>> {
        let ptr = gl::MapBufferRange(
            B::BUFFER_TYPE,                                                 // target
            (offset * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // offset
            (size * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr,   //  length
            gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT,               // usage
        );
        if ptr.is_null() {
            return None;
        }
        return Some(MappedBuffer {
            data: ::std::slice::from_raw_parts_mut(ptr as *mut T, size),
            _marker: ::std::marker::PhantomData,
        });
    }
}

impl<B> Drop for Buffer<B>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

pub struct MappedBuffer<'a, B, DataT: 'a>
where
    B: BufferType,
{
    data: &'a mut [DataT],
    _marker: ::std::marker::PhantomData<B>,
}

impl<'a, B, DataT: 'a> ::std::ops::Deref for MappedBuffer<'a, B, DataT>
where
    B: BufferType,
{
    type Target = [DataT];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, B, DataT: 'a> ::std::ops::DerefMut for MappedBuffer<'a, B, DataT>
where
    B: BufferType,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, B, DataT: 'a> Drop for MappedBuffer<'a, B, DataT>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            gl::UnmapBuffer(B::BUFFER_TYPE);
        }
    }
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl Default for VertexArray {
    fn default() -> Self {
        Self::new()
    }
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
