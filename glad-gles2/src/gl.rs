
pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw;

pub struct FnPtr {
    ptr: *const raw::c_void,
    is_loaded: bool
}

impl FnPtr {
    pub fn empty() -> FnPtr {
        FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false }
    }

    pub fn load<F>(&mut self, loadfn: &mut F, name: &'static str) where F: FnMut(&'static str) -> *const raw::c_void {
        let loaded = loadfn(name);
        if !loaded.is_null() {
            self.ptr = loaded;
            self.is_loaded = true;
        } else {
            self.ptr = FnPtr::not_initialized as *const raw::c_void;
            self.is_loaded = false;
        };
    }

    pub fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            self.ptr = other.ptr;
            self.is_loaded = other.is_loaded;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! { panic!("gles2: function not initialized") }
}

pub mod types {
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use std::os::raw;

pub type GLvoid = raw::c_void;

pub type GLbyte = raw::c_char;
pub type GLubyte = raw::c_uchar;
pub type GLchar = raw::c_char;
pub type GLboolean = raw::c_uchar;

pub type GLshort = raw::c_short;
pub type GLushort = raw::c_ushort;

pub type GLint = raw::c_int;
pub type GLuint = raw::c_uint;
pub type GLint64 = i64;
pub type GLuint64 = u64;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub type GLsizei = GLint;
pub type GLclampx = raw::c_int;
pub type GLfixed = GLint;
pub type GLhalf = raw::c_ushort;
pub type GLhalfNV = raw::c_ushort;
pub type GLhalfARB = raw::c_ushort;

pub type GLenum = raw::c_uint;
pub type GLbitfield = raw::c_uint;

pub type GLfloat = raw::c_float;
pub type GLdouble = raw::c_double;
pub type GLclampf = raw::c_float;
pub type GLclampd = raw::c_double;

pub type GLcharARB = raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = raw::c_uint;

pub enum __GLsync {}

pub type GLsync = *const __GLsync;

pub enum _cl_context {}

pub enum _cl_event {}

pub type GLvdpauSurfaceNV = GLintptr;
pub type GLeglClientBufferEXT = *const raw::c_void;
pub type GLeglImageOES = *const raw::c_void;


pub type GLDEBUGPROC = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLDEBUGPROCAMD = extern "system" fn (
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLVULKANPROCNV = extern "system" fn ();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std;
    use super::types::*;

    pub const GL_ACTIVE_ATTRIBUTES: std::os::raw::c_uint = 0x8B89;
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: std::os::raw::c_uint = 0x8B8A;
    pub const GL_ACTIVE_TEXTURE: std::os::raw::c_uint = 0x84E0;
    pub const GL_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8B86;
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: std::os::raw::c_uint = 0x8B87;
    pub const GL_ALIASED_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x846E;
    pub const GL_ALIASED_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x846D;
    pub const GL_ALPHA: std::os::raw::c_uint = 0x1906;
    pub const GL_ALPHA_BITS: std::os::raw::c_uint = 0x0D55;
    pub const GL_ALWAYS: std::os::raw::c_uint = 0x0207;
    pub const GL_ARRAY_BUFFER: std::os::raw::c_uint = 0x8892;
    pub const GL_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8894;
    pub const GL_ATTACHED_SHADERS: std::os::raw::c_uint = 0x8B85;
    pub const GL_BACK: std::os::raw::c_uint = 0x0405;
    pub const GL_BLEND: std::os::raw::c_uint = 0x0BE2;
    pub const GL_BLEND_COLOR: std::os::raw::c_uint = 0x8005;
    pub const GL_BLEND_DST_ALPHA: std::os::raw::c_uint = 0x80CA;
    pub const GL_BLEND_DST_RGB: std::os::raw::c_uint = 0x80C8;
    pub const GL_BLEND_EQUATION: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_EQUATION_ALPHA: std::os::raw::c_uint = 0x883D;
    pub const GL_BLEND_EQUATION_RGB: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_SRC_ALPHA: std::os::raw::c_uint = 0x80CB;
    pub const GL_BLEND_SRC_RGB: std::os::raw::c_uint = 0x80C9;
    pub const GL_BLUE_BITS: std::os::raw::c_uint = 0x0D54;
    pub const GL_BOOL: std::os::raw::c_uint = 0x8B56;
    pub const GL_BOOL_VEC2: std::os::raw::c_uint = 0x8B57;
    pub const GL_BOOL_VEC3: std::os::raw::c_uint = 0x8B58;
    pub const GL_BOOL_VEC4: std::os::raw::c_uint = 0x8B59;
    pub const GL_BUFFER_SIZE: std::os::raw::c_uint = 0x8764;
    pub const GL_BUFFER_USAGE: std::os::raw::c_uint = 0x8765;
    pub const GL_BYTE: std::os::raw::c_uint = 0x1400;
    pub const GL_CCW: std::os::raw::c_uint = 0x0901;
    pub const GL_CLAMP_TO_EDGE: std::os::raw::c_uint = 0x812F;
    pub const GL_COLOR_ATTACHMENT0: std::os::raw::c_uint = 0x8CE0;
    pub const GL_COLOR_BUFFER_BIT: std::os::raw::c_uint = 0x00004000;
    pub const GL_COLOR_CLEAR_VALUE: std::os::raw::c_uint = 0x0C22;
    pub const GL_COLOR_WRITEMASK: std::os::raw::c_uint = 0x0C23;
    pub const GL_COMPILE_STATUS: std::os::raw::c_uint = 0x8B81;
    pub const GL_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A3;
    pub const GL_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8003;
    pub const GL_CONSTANT_COLOR: std::os::raw::c_uint = 0x8001;
    pub const GL_CULL_FACE: std::os::raw::c_uint = 0x0B44;
    pub const GL_CULL_FACE_MODE: std::os::raw::c_uint = 0x0B45;
    pub const GL_CURRENT_PROGRAM: std::os::raw::c_uint = 0x8B8D;
    pub const GL_CURRENT_VERTEX_ATTRIB: std::os::raw::c_uint = 0x8626;
    pub const GL_CW: std::os::raw::c_uint = 0x0900;
    pub const GL_DECR: std::os::raw::c_uint = 0x1E03;
    pub const GL_DECR_WRAP: std::os::raw::c_uint = 0x8508;
    pub const GL_DELETE_STATUS: std::os::raw::c_uint = 0x8B80;
    pub const GL_DEPTH_ATTACHMENT: std::os::raw::c_uint = 0x8D00;
    pub const GL_DEPTH_BITS: std::os::raw::c_uint = 0x0D56;
    pub const GL_DEPTH_BUFFER_BIT: std::os::raw::c_uint = 0x00000100;
    pub const GL_DEPTH_CLEAR_VALUE: std::os::raw::c_uint = 0x0B73;
    pub const GL_DEPTH_COMPONENT: std::os::raw::c_uint = 0x1902;
    pub const GL_DEPTH_COMPONENT16: std::os::raw::c_uint = 0x81A5;
    pub const GL_DEPTH_FUNC: std::os::raw::c_uint = 0x0B74;
    pub const GL_DEPTH_RANGE: std::os::raw::c_uint = 0x0B70;
    pub const GL_DEPTH_TEST: std::os::raw::c_uint = 0x0B71;
    pub const GL_DEPTH_WRITEMASK: std::os::raw::c_uint = 0x0B72;
    pub const GL_DITHER: std::os::raw::c_uint = 0x0BD0;
    pub const GL_DONT_CARE: std::os::raw::c_uint = 0x1100;
    pub const GL_DST_ALPHA: std::os::raw::c_uint = 0x0304;
    pub const GL_DST_COLOR: std::os::raw::c_uint = 0x0306;
    pub const GL_DYNAMIC_DRAW: std::os::raw::c_uint = 0x88E8;
    pub const GL_ELEMENT_ARRAY_BUFFER: std::os::raw::c_uint = 0x8893;
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8895;
    pub const GL_EQUAL: std::os::raw::c_uint = 0x0202;
    pub const GL_EXTENSIONS: std::os::raw::c_uint = 0x1F03;
    pub const GL_FALSE: std::os::raw::c_uchar = 0;
    pub const GL_FASTEST: std::os::raw::c_uint = 0x1101;
    pub const GL_FIXED: std::os::raw::c_uint = 0x140C;
    pub const GL_FLOAT: std::os::raw::c_uint = 0x1406;
    pub const GL_FLOAT_MAT2: std::os::raw::c_uint = 0x8B5A;
    pub const GL_FLOAT_MAT3: std::os::raw::c_uint = 0x8B5B;
    pub const GL_FLOAT_MAT4: std::os::raw::c_uint = 0x8B5C;
    pub const GL_FLOAT_VEC2: std::os::raw::c_uint = 0x8B50;
    pub const GL_FLOAT_VEC3: std::os::raw::c_uint = 0x8B51;
    pub const GL_FLOAT_VEC4: std::os::raw::c_uint = 0x8B52;
    pub const GL_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8B30;
    pub const GL_FRAMEBUFFER: std::os::raw::c_uint = 0x8D40;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: std::os::raw::c_uint = 0x8CD1;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: std::os::raw::c_uint = 0x8CD0;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: std::os::raw::c_uint = 0x8CD3;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: std::os::raw::c_uint = 0x8CD2;
    pub const GL_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CA6;
    pub const GL_FRAMEBUFFER_COMPLETE: std::os::raw::c_uint = 0x8CD5;
    pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: std::os::raw::c_uint = 0x8CD6;
    pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: std::os::raw::c_uint = 0x8CD9;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: std::os::raw::c_uint = 0x8CD7;
    pub const GL_FRAMEBUFFER_UNSUPPORTED: std::os::raw::c_uint = 0x8CDD;
    pub const GL_FRONT: std::os::raw::c_uint = 0x0404;
    pub const GL_FRONT_AND_BACK: std::os::raw::c_uint = 0x0408;
    pub const GL_FRONT_FACE: std::os::raw::c_uint = 0x0B46;
    pub const GL_FUNC_ADD: std::os::raw::c_uint = 0x8006;
    pub const GL_FUNC_REVERSE_SUBTRACT: std::os::raw::c_uint = 0x800B;
    pub const GL_FUNC_SUBTRACT: std::os::raw::c_uint = 0x800A;
    pub const GL_GENERATE_MIPMAP_HINT: std::os::raw::c_uint = 0x8192;
    pub const GL_GEQUAL: std::os::raw::c_uint = 0x0206;
    pub const GL_GREATER: std::os::raw::c_uint = 0x0204;
    pub const GL_GREEN_BITS: std::os::raw::c_uint = 0x0D53;
    pub const GL_HIGH_FLOAT: std::os::raw::c_uint = 0x8DF2;
    pub const GL_HIGH_INT: std::os::raw::c_uint = 0x8DF5;
    pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: std::os::raw::c_uint = 0x8B9B;
    pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: std::os::raw::c_uint = 0x8B9A;
    pub const GL_INCR: std::os::raw::c_uint = 0x1E02;
    pub const GL_INCR_WRAP: std::os::raw::c_uint = 0x8507;
    pub const GL_INFO_LOG_LENGTH: std::os::raw::c_uint = 0x8B84;
    pub const GL_INT: std::os::raw::c_uint = 0x1404;
    pub const GL_INT_VEC2: std::os::raw::c_uint = 0x8B53;
    pub const GL_INT_VEC3: std::os::raw::c_uint = 0x8B54;
    pub const GL_INT_VEC4: std::os::raw::c_uint = 0x8B55;
    pub const GL_INVALID_ENUM: std::os::raw::c_uint = 0x0500;
    pub const GL_INVALID_FRAMEBUFFER_OPERATION: std::os::raw::c_uint = 0x0506;
    pub const GL_INVALID_OPERATION: std::os::raw::c_uint = 0x0502;
    pub const GL_INVALID_VALUE: std::os::raw::c_uint = 0x0501;
    pub const GL_INVERT: std::os::raw::c_uint = 0x150A;
    pub const GL_KEEP: std::os::raw::c_uint = 0x1E00;
    pub const GL_LEQUAL: std::os::raw::c_uint = 0x0203;
    pub const GL_LESS: std::os::raw::c_uint = 0x0201;
    pub const GL_LINEAR: std::os::raw::c_uint = 0x2601;
    pub const GL_LINEAR_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2703;
    pub const GL_LINEAR_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2701;
    pub const GL_LINES: std::os::raw::c_uint = 0x0001;
    pub const GL_LINE_LOOP: std::os::raw::c_uint = 0x0002;
    pub const GL_LINE_STRIP: std::os::raw::c_uint = 0x0003;
    pub const GL_LINE_WIDTH: std::os::raw::c_uint = 0x0B21;
    pub const GL_LINK_STATUS: std::os::raw::c_uint = 0x8B82;
    pub const GL_LOW_FLOAT: std::os::raw::c_uint = 0x8DF0;
    pub const GL_LOW_INT: std::os::raw::c_uint = 0x8DF3;
    pub const GL_LUMINANCE: std::os::raw::c_uint = 0x1909;
    pub const GL_LUMINANCE_ALPHA: std::os::raw::c_uint = 0x190A;
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4D;
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: std::os::raw::c_uint = 0x851C;
    pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: std::os::raw::c_uint = 0x8DFD;
    pub const GL_MAX_RENDERBUFFER_SIZE: std::os::raw::c_uint = 0x84E8;
    pub const GL_MAX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8872;
    pub const GL_MAX_TEXTURE_SIZE: std::os::raw::c_uint = 0x0D33;
    pub const GL_MAX_VARYING_VECTORS: std::os::raw::c_uint = 0x8DFC;
    pub const GL_MAX_VERTEX_ATTRIBS: std::os::raw::c_uint = 0x8869;
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4C;
    pub const GL_MAX_VERTEX_UNIFORM_VECTORS: std::os::raw::c_uint = 0x8DFB;
    pub const GL_MAX_VIEWPORT_DIMS: std::os::raw::c_uint = 0x0D3A;
    pub const GL_MEDIUM_FLOAT: std::os::raw::c_uint = 0x8DF1;
    pub const GL_MEDIUM_INT: std::os::raw::c_uint = 0x8DF4;
    pub const GL_MIRRORED_REPEAT: std::os::raw::c_uint = 0x8370;
    pub const GL_NEAREST: std::os::raw::c_uint = 0x2600;
    pub const GL_NEAREST_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2702;
    pub const GL_NEAREST_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2700;
    pub const GL_NEVER: std::os::raw::c_uint = 0x0200;
    pub const GL_NICEST: std::os::raw::c_uint = 0x1102;
    pub const GL_NONE: std::os::raw::c_uint = 0;
    pub const GL_NOTEQUAL: std::os::raw::c_uint = 0x0205;
    pub const GL_NO_ERROR: std::os::raw::c_uint = 0;
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A2;
    pub const GL_NUM_SHADER_BINARY_FORMATS: std::os::raw::c_uint = 0x8DF9;
    pub const GL_ONE: std::os::raw::c_uint = 1;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8004;
    pub const GL_ONE_MINUS_CONSTANT_COLOR: std::os::raw::c_uint = 0x8002;
    pub const GL_ONE_MINUS_DST_ALPHA: std::os::raw::c_uint = 0x0305;
    pub const GL_ONE_MINUS_DST_COLOR: std::os::raw::c_uint = 0x0307;
    pub const GL_ONE_MINUS_SRC_ALPHA: std::os::raw::c_uint = 0x0303;
    pub const GL_ONE_MINUS_SRC_COLOR: std::os::raw::c_uint = 0x0301;
    pub const GL_OUT_OF_MEMORY: std::os::raw::c_uint = 0x0505;
    pub const GL_PACK_ALIGNMENT: std::os::raw::c_uint = 0x0D05;
    pub const GL_POINTS: std::os::raw::c_uint = 0x0000;
    pub const GL_POLYGON_OFFSET_FACTOR: std::os::raw::c_uint = 0x8038;
    pub const GL_POLYGON_OFFSET_FILL: std::os::raw::c_uint = 0x8037;
    pub const GL_POLYGON_OFFSET_UNITS: std::os::raw::c_uint = 0x2A00;
    pub const GL_RED_BITS: std::os::raw::c_uint = 0x0D52;
    pub const GL_RENDERBUFFER: std::os::raw::c_uint = 0x8D41;
    pub const GL_RENDERBUFFER_ALPHA_SIZE: std::os::raw::c_uint = 0x8D53;
    pub const GL_RENDERBUFFER_BINDING: std::os::raw::c_uint = 0x8CA7;
    pub const GL_RENDERBUFFER_BLUE_SIZE: std::os::raw::c_uint = 0x8D52;
    pub const GL_RENDERBUFFER_DEPTH_SIZE: std::os::raw::c_uint = 0x8D54;
    pub const GL_RENDERBUFFER_GREEN_SIZE: std::os::raw::c_uint = 0x8D51;
    pub const GL_RENDERBUFFER_HEIGHT: std::os::raw::c_uint = 0x8D43;
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT: std::os::raw::c_uint = 0x8D44;
    pub const GL_RENDERBUFFER_RED_SIZE: std::os::raw::c_uint = 0x8D50;
    pub const GL_RENDERBUFFER_STENCIL_SIZE: std::os::raw::c_uint = 0x8D55;
    pub const GL_RENDERBUFFER_WIDTH: std::os::raw::c_uint = 0x8D42;
    pub const GL_RENDERER: std::os::raw::c_uint = 0x1F01;
    pub const GL_REPEAT: std::os::raw::c_uint = 0x2901;
    pub const GL_REPLACE: std::os::raw::c_uint = 0x1E01;
    pub const GL_RGB: std::os::raw::c_uint = 0x1907;
    pub const GL_RGB565: std::os::raw::c_uint = 0x8D62;
    pub const GL_RGB5_A1: std::os::raw::c_uint = 0x8057;
    pub const GL_RGBA: std::os::raw::c_uint = 0x1908;
    pub const GL_RGBA4: std::os::raw::c_uint = 0x8056;
    pub const GL_SAMPLER_2D: std::os::raw::c_uint = 0x8B5E;
    pub const GL_SAMPLER_CUBE: std::os::raw::c_uint = 0x8B60;
    pub const GL_SAMPLES: std::os::raw::c_uint = 0x80A9;
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE: std::os::raw::c_uint = 0x809E;
    pub const GL_SAMPLE_BUFFERS: std::os::raw::c_uint = 0x80A8;
    pub const GL_SAMPLE_COVERAGE: std::os::raw::c_uint = 0x80A0;
    pub const GL_SAMPLE_COVERAGE_INVERT: std::os::raw::c_uint = 0x80AB;
    pub const GL_SAMPLE_COVERAGE_VALUE: std::os::raw::c_uint = 0x80AA;
    pub const GL_SCISSOR_BOX: std::os::raw::c_uint = 0x0C10;
    pub const GL_SCISSOR_TEST: std::os::raw::c_uint = 0x0C11;
    pub const GL_SHADER_BINARY_FORMATS: std::os::raw::c_uint = 0x8DF8;
    pub const GL_SHADER_COMPILER: std::os::raw::c_uint = 0x8DFA;
    pub const GL_SHADER_SOURCE_LENGTH: std::os::raw::c_uint = 0x8B88;
    pub const GL_SHADER_TYPE: std::os::raw::c_uint = 0x8B4F;
    pub const GL_SHADING_LANGUAGE_VERSION: std::os::raw::c_uint = 0x8B8C;
    pub const GL_SHORT: std::os::raw::c_uint = 0x1402;
    pub const GL_SRC_ALPHA: std::os::raw::c_uint = 0x0302;
    pub const GL_SRC_ALPHA_SATURATE: std::os::raw::c_uint = 0x0308;
    pub const GL_SRC_COLOR: std::os::raw::c_uint = 0x0300;
    pub const GL_STATIC_DRAW: std::os::raw::c_uint = 0x88E4;
    pub const GL_STENCIL_ATTACHMENT: std::os::raw::c_uint = 0x8D20;
    pub const GL_STENCIL_BACK_FAIL: std::os::raw::c_uint = 0x8801;
    pub const GL_STENCIL_BACK_FUNC: std::os::raw::c_uint = 0x8800;
    pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x8802;
    pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x8803;
    pub const GL_STENCIL_BACK_REF: std::os::raw::c_uint = 0x8CA3;
    pub const GL_STENCIL_BACK_VALUE_MASK: std::os::raw::c_uint = 0x8CA4;
    pub const GL_STENCIL_BACK_WRITEMASK: std::os::raw::c_uint = 0x8CA5;
    pub const GL_STENCIL_BITS: std::os::raw::c_uint = 0x0D57;
    pub const GL_STENCIL_BUFFER_BIT: std::os::raw::c_uint = 0x00000400;
    pub const GL_STENCIL_CLEAR_VALUE: std::os::raw::c_uint = 0x0B91;
    pub const GL_STENCIL_FAIL: std::os::raw::c_uint = 0x0B94;
    pub const GL_STENCIL_FUNC: std::os::raw::c_uint = 0x0B92;
    pub const GL_STENCIL_INDEX8: std::os::raw::c_uint = 0x8D48;
    pub const GL_STENCIL_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x0B96;
    pub const GL_STENCIL_REF: std::os::raw::c_uint = 0x0B97;
    pub const GL_STENCIL_TEST: std::os::raw::c_uint = 0x0B90;
    pub const GL_STENCIL_VALUE_MASK: std::os::raw::c_uint = 0x0B93;
    pub const GL_STENCIL_WRITEMASK: std::os::raw::c_uint = 0x0B98;
    pub const GL_STREAM_DRAW: std::os::raw::c_uint = 0x88E0;
    pub const GL_SUBPIXEL_BITS: std::os::raw::c_uint = 0x0D50;
    pub const GL_TEXTURE: std::os::raw::c_uint = 0x1702;
    pub const GL_TEXTURE0: std::os::raw::c_uint = 0x84C0;
    pub const GL_TEXTURE1: std::os::raw::c_uint = 0x84C1;
    pub const GL_TEXTURE10: std::os::raw::c_uint = 0x84CA;
    pub const GL_TEXTURE11: std::os::raw::c_uint = 0x84CB;
    pub const GL_TEXTURE12: std::os::raw::c_uint = 0x84CC;
    pub const GL_TEXTURE13: std::os::raw::c_uint = 0x84CD;
    pub const GL_TEXTURE14: std::os::raw::c_uint = 0x84CE;
    pub const GL_TEXTURE15: std::os::raw::c_uint = 0x84CF;
    pub const GL_TEXTURE16: std::os::raw::c_uint = 0x84D0;
    pub const GL_TEXTURE17: std::os::raw::c_uint = 0x84D1;
    pub const GL_TEXTURE18: std::os::raw::c_uint = 0x84D2;
    pub const GL_TEXTURE19: std::os::raw::c_uint = 0x84D3;
    pub const GL_TEXTURE2: std::os::raw::c_uint = 0x84C2;
    pub const GL_TEXTURE20: std::os::raw::c_uint = 0x84D4;
    pub const GL_TEXTURE21: std::os::raw::c_uint = 0x84D5;
    pub const GL_TEXTURE22: std::os::raw::c_uint = 0x84D6;
    pub const GL_TEXTURE23: std::os::raw::c_uint = 0x84D7;
    pub const GL_TEXTURE24: std::os::raw::c_uint = 0x84D8;
    pub const GL_TEXTURE25: std::os::raw::c_uint = 0x84D9;
    pub const GL_TEXTURE26: std::os::raw::c_uint = 0x84DA;
    pub const GL_TEXTURE27: std::os::raw::c_uint = 0x84DB;
    pub const GL_TEXTURE28: std::os::raw::c_uint = 0x84DC;
    pub const GL_TEXTURE29: std::os::raw::c_uint = 0x84DD;
    pub const GL_TEXTURE3: std::os::raw::c_uint = 0x84C3;
    pub const GL_TEXTURE30: std::os::raw::c_uint = 0x84DE;
    pub const GL_TEXTURE31: std::os::raw::c_uint = 0x84DF;
    pub const GL_TEXTURE4: std::os::raw::c_uint = 0x84C4;
    pub const GL_TEXTURE5: std::os::raw::c_uint = 0x84C5;
    pub const GL_TEXTURE6: std::os::raw::c_uint = 0x84C6;
    pub const GL_TEXTURE7: std::os::raw::c_uint = 0x84C7;
    pub const GL_TEXTURE8: std::os::raw::c_uint = 0x84C8;
    pub const GL_TEXTURE9: std::os::raw::c_uint = 0x84C9;
    pub const GL_TEXTURE_2D: std::os::raw::c_uint = 0x0DE1;
    pub const GL_TEXTURE_BINDING_2D: std::os::raw::c_uint = 0x8069;
    pub const GL_TEXTURE_BINDING_CUBE_MAP: std::os::raw::c_uint = 0x8514;
    pub const GL_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x8513;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: std::os::raw::c_uint = 0x8516;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: std::os::raw::c_uint = 0x8518;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: std::os::raw::c_uint = 0x851A;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: std::os::raw::c_uint = 0x8515;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: std::os::raw::c_uint = 0x8517;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: std::os::raw::c_uint = 0x8519;
    pub const GL_TEXTURE_MAG_FILTER: std::os::raw::c_uint = 0x2800;
    pub const GL_TEXTURE_MIN_FILTER: std::os::raw::c_uint = 0x2801;
    pub const GL_TEXTURE_WRAP_S: std::os::raw::c_uint = 0x2802;
    pub const GL_TEXTURE_WRAP_T: std::os::raw::c_uint = 0x2803;
    pub const GL_TRIANGLES: std::os::raw::c_uint = 0x0004;
    pub const GL_TRIANGLE_FAN: std::os::raw::c_uint = 0x0006;
    pub const GL_TRIANGLE_STRIP: std::os::raw::c_uint = 0x0005;
    pub const GL_TRUE: std::os::raw::c_uchar = 1;
    pub const GL_UNPACK_ALIGNMENT: std::os::raw::c_uint = 0x0CF5;
    pub const GL_UNSIGNED_BYTE: std::os::raw::c_uint = 0x1401;
    pub const GL_UNSIGNED_INT: std::os::raw::c_uint = 0x1405;
    pub const GL_UNSIGNED_SHORT: std::os::raw::c_uint = 0x1403;
    pub const GL_UNSIGNED_SHORT_4_4_4_4: std::os::raw::c_uint = 0x8033;
    pub const GL_UNSIGNED_SHORT_5_5_5_1: std::os::raw::c_uint = 0x8034;
    pub const GL_UNSIGNED_SHORT_5_6_5: std::os::raw::c_uint = 0x8363;
    pub const GL_VALIDATE_STATUS: std::os::raw::c_uint = 0x8B83;
    pub const GL_VENDOR: std::os::raw::c_uint = 0x1F00;
    pub const GL_VERSION: std::os::raw::c_uint = 0x1F02;
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889F;
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: std::os::raw::c_uint = 0x8622;
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: std::os::raw::c_uint = 0x886A;
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: std::os::raw::c_uint = 0x8645;
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: std::os::raw::c_uint = 0x8623;
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: std::os::raw::c_uint = 0x8624;
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: std::os::raw::c_uint = 0x8625;
    pub const GL_VERTEX_SHADER: std::os::raw::c_uint = 0x8B31;
    pub const GL_VIEWPORT: std::os::raw::c_uint = 0x0BA2;
    pub const GL_ZERO: std::os::raw::c_uint = 0;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code)]

    use std;
    use std::mem;
    use super::storage;
    use super::types::*;

     #[inline] pub unsafe fn ActiveTexture(texture: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ActiveTexture.ptr)(texture) }
     #[inline] pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::AttachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindAttribLocation.ptr)(program, index, name) }
     #[inline] pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindBuffer.ptr)(target, buffer) }
     #[inline] pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindFramebuffer.ptr)(target, framebuffer) }
     #[inline] pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindRenderbuffer.ptr)(target, renderbuffer) }
     #[inline] pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTexture.ptr)(target, texture) }
     #[inline] pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::BlendColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn BlendEquation(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BlendEquation.ptr)(mode) }
     #[inline] pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendEquationSeparate.ptr)(modeRGB, modeAlpha) }
     #[inline] pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendFunc.ptr)(sfactor, dfactor) }
     #[inline] pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::BlendFuncSeparate.ptr)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
     #[inline] pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const std::os::raw::c_void, usage: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const std::os::raw::c_void, GLenum) -> ()>(storage::BufferData.ptr)(target, size, data, usage) }
     #[inline] pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const std::os::raw::c_void) -> ()>(storage::BufferSubData.ptr)(target, offset, size, data) }
     #[inline] pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum { mem::transmute::<_, extern "system" fn(GLenum) -> GLenum>(storage::CheckFramebufferStatus.ptr)(target) }
     #[inline] pub unsafe fn Clear(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::Clear.ptr)(mask) }
     #[inline] pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ClearDepthf(d: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::ClearDepthf.ptr)(d) }
     #[inline] pub unsafe fn ClearStencil(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ClearStencil.ptr)(s) }
     #[inline] pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMask.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn CompileShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CompileShader.ptr)(shader) }
     #[inline] pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage2D.ptr)(target, level, internalformat, width, height, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
     #[inline] pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> ()>(storage::CopyTexImage2D.ptr)(target, level, internalformat, x, y, width, height, border) }
     #[inline] pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage2D.ptr)(target, level, xoffset, yoffset, x, y, width, height) }
     #[inline] pub unsafe fn CreateProgram() -> GLuint { mem::transmute::<_, extern "system" fn() -> GLuint>(storage::CreateProgram.ptr)() }
     #[inline] pub unsafe fn CreateShader(type_: GLenum) -> GLuint { mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(storage::CreateShader.ptr)(type_) }
     #[inline] pub unsafe fn CullFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::CullFace.ptr)(mode) }
     #[inline] pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteFramebuffers.ptr)(n, framebuffers) }
     #[inline] pub unsafe fn DeleteProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteProgram.ptr)(program) }
     #[inline] pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteRenderbuffers.ptr)(n, renderbuffers) }
     #[inline] pub unsafe fn DeleteShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteShader.ptr)(shader) }
     #[inline] pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn DepthFunc(func: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DepthFunc.ptr)(func) }
     #[inline] pub unsafe fn DepthMask(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::DepthMask.ptr)(flag) }
     #[inline] pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::DepthRangef.ptr)(n, f) }
     #[inline] pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DetachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn Disable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Disable.ptr)(cap) }
     #[inline] pub unsafe fn DisableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DisableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(storage::DrawArrays.ptr)(mode, first, count) }
     #[inline] pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawElements.ptr)(mode, count, type_, indices) }
     #[inline] pub unsafe fn Enable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Enable.ptr)(cap) }
     #[inline] pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::EnableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn Finish() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.ptr)() }
     #[inline] pub unsafe fn Flush() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.ptr)() }
     #[inline] pub unsafe fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> ()>(storage::FramebufferRenderbuffer.ptr)(target, attachment, renderbuffertarget, renderbuffer) }
     #[inline] pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture2D.ptr)(target, attachment, textarget, texture, level) }
     #[inline] pub unsafe fn FrontFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::FrontFace.ptr)(mode) }
     #[inline] pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenFramebuffers.ptr)(n, framebuffers) }
     #[inline] pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenRenderbuffers.ptr)(n, renderbuffers) }
     #[inline] pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn GenerateMipmap(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::GenerateMipmap.ptr)(target) }
     #[inline] pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveAttrib.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveUniform.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>(storage::GetAttachedShaders.ptr)(program, maxCount, count, shaders) }
     #[inline] pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetAttribLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(storage::GetBooleanv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetBufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetError() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetError.ptr)() }
     #[inline] pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetFloatv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) -> ()>(storage::GetFramebufferAttachmentParameteriv.ptr)(target, attachment, pname, params) }
     #[inline] pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(storage::GetIntegerv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramInfoLog.ptr)(program, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetProgramiv.ptr)(program, pname, params) }
     #[inline] pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetRenderbufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderInfoLog.ptr)(shader, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint) -> ()>(storage::GetShaderPrecisionFormat.ptr)(shadertype, precisiontype, range, precision) }
     #[inline] pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderSource.ptr)(shader, bufSize, length, source) }
     #[inline] pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetShaderiv.ptr)(shader, pname, params) }
     #[inline] pub unsafe fn GetString(name: GLenum) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(storage::GetString.ptr)(name) }
     #[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetUniformLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>(storage::GetUniformfv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>(storage::GetUniformiv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetVertexAttribPointerv.ptr)(index, pname, pointer) }
     #[inline] pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetVertexAttribfv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::Hint.ptr)(target, mode) }
     #[inline] pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsBuffer.ptr)(buffer) }
     #[inline] pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::IsEnabled.ptr)(cap) }
     #[inline] pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsFramebuffer.ptr)(framebuffer) }
     #[inline] pub unsafe fn IsProgram(program: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgram.ptr)(program) }
     #[inline] pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsRenderbuffer.ptr)(renderbuffer) }
     #[inline] pub unsafe fn IsShader(shader: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsShader.ptr)(shader) }
     #[inline] pub unsafe fn IsTexture(texture: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTexture.ptr)(texture) }
     #[inline] pub unsafe fn LineWidth(width: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::LineWidth.ptr)(width) }
     #[inline] pub unsafe fn LinkProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LinkProgram.ptr)(program) }
     #[inline] pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelStorei.ptr)(pname, param) }
     #[inline] pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PolygonOffset.ptr)(factor, units) }
     #[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::ReadPixels.ptr)(x, y, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn ReleaseShaderCompiler() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::ReleaseShaderCompiler.ptr)() }
     #[inline] pub unsafe fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) -> ()>(storage::RenderbufferStorage.ptr)(target, internalformat, width, height) }
     #[inline] pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>(storage::SampleCoverage.ptr)(value, invert) }
     #[inline] pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Scissor.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const std::os::raw::c_void, length: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint, GLenum, *const std::os::raw::c_void, GLsizei) -> ()>(storage::ShaderBinary.ptr)(count, shaders, binaryformat, binary, length) }
     #[inline] pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>(storage::ShaderSource.ptr)(shader, count, string, length) }
     #[inline] pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(storage::StencilFunc.ptr)(func, ref_, mask) }
     #[inline] pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>(storage::StencilFuncSeparate.ptr)(face, func, ref_, mask) }
     #[inline] pub unsafe fn StencilMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::StencilMask.ptr)(mask) }
     #[inline] pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::StencilMaskSeparate.ptr)(face, mask) }
     #[inline] pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(storage::StencilOp.ptr)(fail, zfail, zpass) }
     #[inline] pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::StencilOpSeparate.ptr)(face, sfail, dpfail, dppass) }
     #[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage2D.ptr)(target, level, internalformat, width, height, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexParameterf.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexParameteri.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(storage::Uniform1f.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform1fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform1i(location: GLint, v0: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::Uniform1i.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform1iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(storage::Uniform2f.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform2fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Uniform2i.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform2iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform3f.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform3fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Uniform3i.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform3iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform4f.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform4fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint) -> ()>(storage::Uniform4i.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform4iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UseProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::UseProgram.ptr)(program) }
     #[inline] pub unsafe fn ValidateProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgram.ptr)(program) }
     #[inline] pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>(storage::VertexAttrib1f.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib1fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>(storage::VertexAttrib2f.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib2fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib3f.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib3fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib4f.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib4fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexAttribPointer.ptr)(index, size, type_, normalized, stride, pointer) }
     #[inline] pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Viewport.ptr)(x, y, width, height) }
}

mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw;

     pub static mut ActiveTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut AttachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquationSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CheckFramebufferStatus: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Clear: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearDepthf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearStencil: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompileShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CullFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteFramebuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteRenderbuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRangef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DetachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Disable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DisableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Enable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EnableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Finish: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Flush: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FrontFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenFramebuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenRenderbuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenerateMipmap: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniform: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttachedShaders: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBooleanv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetError: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFloatv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFramebufferAttachmentParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetIntegerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetRenderbufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderPrecisionFormat: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetString: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Hint: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsEnabled: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LineWidth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LinkProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStorei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonOffset: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReleaseShaderCompiler: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RenderbufferStorage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SampleCoverage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scissor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShaderBinary: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMaskSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOpSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UseProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ValidateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Viewport: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
}

pub fn load<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
    unsafe {
         storage::ActiveTexture.load(&mut loadfn, "glActiveTexture");
         storage::AttachShader.load(&mut loadfn, "glAttachShader");
         storage::BindAttribLocation.load(&mut loadfn, "glBindAttribLocation");
         storage::BindBuffer.load(&mut loadfn, "glBindBuffer");
         storage::BindFramebuffer.load(&mut loadfn, "glBindFramebuffer");
         storage::BindRenderbuffer.load(&mut loadfn, "glBindRenderbuffer");
         storage::BindTexture.load(&mut loadfn, "glBindTexture");
         storage::BlendColor.load(&mut loadfn, "glBlendColor");
         storage::BlendEquation.load(&mut loadfn, "glBlendEquation");
         storage::BlendEquationSeparate.load(&mut loadfn, "glBlendEquationSeparate");
         storage::BlendFunc.load(&mut loadfn, "glBlendFunc");
         storage::BlendFuncSeparate.load(&mut loadfn, "glBlendFuncSeparate");
         storage::BufferData.load(&mut loadfn, "glBufferData");
         storage::BufferSubData.load(&mut loadfn, "glBufferSubData");
         storage::CheckFramebufferStatus.load(&mut loadfn, "glCheckFramebufferStatus");
         storage::Clear.load(&mut loadfn, "glClear");
         storage::ClearColor.load(&mut loadfn, "glClearColor");
         storage::ClearDepthf.load(&mut loadfn, "glClearDepthf");
         storage::ClearStencil.load(&mut loadfn, "glClearStencil");
         storage::ColorMask.load(&mut loadfn, "glColorMask");
         storage::CompileShader.load(&mut loadfn, "glCompileShader");
         storage::CompressedTexImage2D.load(&mut loadfn, "glCompressedTexImage2D");
         storage::CompressedTexSubImage2D.load(&mut loadfn, "glCompressedTexSubImage2D");
         storage::CopyTexImage2D.load(&mut loadfn, "glCopyTexImage2D");
         storage::CopyTexSubImage2D.load(&mut loadfn, "glCopyTexSubImage2D");
         storage::CreateProgram.load(&mut loadfn, "glCreateProgram");
         storage::CreateShader.load(&mut loadfn, "glCreateShader");
         storage::CullFace.load(&mut loadfn, "glCullFace");
         storage::DeleteBuffers.load(&mut loadfn, "glDeleteBuffers");
         storage::DeleteFramebuffers.load(&mut loadfn, "glDeleteFramebuffers");
         storage::DeleteProgram.load(&mut loadfn, "glDeleteProgram");
         storage::DeleteRenderbuffers.load(&mut loadfn, "glDeleteRenderbuffers");
         storage::DeleteShader.load(&mut loadfn, "glDeleteShader");
         storage::DeleteTextures.load(&mut loadfn, "glDeleteTextures");
         storage::DepthFunc.load(&mut loadfn, "glDepthFunc");
         storage::DepthMask.load(&mut loadfn, "glDepthMask");
         storage::DepthRangef.load(&mut loadfn, "glDepthRangef");
         storage::DetachShader.load(&mut loadfn, "glDetachShader");
         storage::Disable.load(&mut loadfn, "glDisable");
         storage::DisableVertexAttribArray.load(&mut loadfn, "glDisableVertexAttribArray");
         storage::DrawArrays.load(&mut loadfn, "glDrawArrays");
         storage::DrawElements.load(&mut loadfn, "glDrawElements");
         storage::Enable.load(&mut loadfn, "glEnable");
         storage::EnableVertexAttribArray.load(&mut loadfn, "glEnableVertexAttribArray");
         storage::Finish.load(&mut loadfn, "glFinish");
         storage::Flush.load(&mut loadfn, "glFlush");
         storage::FramebufferRenderbuffer.load(&mut loadfn, "glFramebufferRenderbuffer");
         storage::FramebufferTexture2D.load(&mut loadfn, "glFramebufferTexture2D");
         storage::FrontFace.load(&mut loadfn, "glFrontFace");
         storage::GenBuffers.load(&mut loadfn, "glGenBuffers");
         storage::GenFramebuffers.load(&mut loadfn, "glGenFramebuffers");
         storage::GenRenderbuffers.load(&mut loadfn, "glGenRenderbuffers");
         storage::GenTextures.load(&mut loadfn, "glGenTextures");
         storage::GenerateMipmap.load(&mut loadfn, "glGenerateMipmap");
         storage::GetActiveAttrib.load(&mut loadfn, "glGetActiveAttrib");
         storage::GetActiveUniform.load(&mut loadfn, "glGetActiveUniform");
         storage::GetAttachedShaders.load(&mut loadfn, "glGetAttachedShaders");
         storage::GetAttribLocation.load(&mut loadfn, "glGetAttribLocation");
         storage::GetBooleanv.load(&mut loadfn, "glGetBooleanv");
         storage::GetBufferParameteriv.load(&mut loadfn, "glGetBufferParameteriv");
         storage::GetError.load(&mut loadfn, "glGetError");
         storage::GetFloatv.load(&mut loadfn, "glGetFloatv");
         storage::GetFramebufferAttachmentParameteriv.load(&mut loadfn, "glGetFramebufferAttachmentParameteriv");
         storage::GetIntegerv.load(&mut loadfn, "glGetIntegerv");
         storage::GetProgramInfoLog.load(&mut loadfn, "glGetProgramInfoLog");
         storage::GetProgramiv.load(&mut loadfn, "glGetProgramiv");
         storage::GetRenderbufferParameteriv.load(&mut loadfn, "glGetRenderbufferParameteriv");
         storage::GetShaderInfoLog.load(&mut loadfn, "glGetShaderInfoLog");
         storage::GetShaderPrecisionFormat.load(&mut loadfn, "glGetShaderPrecisionFormat");
         storage::GetShaderSource.load(&mut loadfn, "glGetShaderSource");
         storage::GetShaderiv.load(&mut loadfn, "glGetShaderiv");
         storage::GetString.load(&mut loadfn, "glGetString");
         storage::GetTexParameterfv.load(&mut loadfn, "glGetTexParameterfv");
         storage::GetTexParameteriv.load(&mut loadfn, "glGetTexParameteriv");
         storage::GetUniformLocation.load(&mut loadfn, "glGetUniformLocation");
         storage::GetUniformfv.load(&mut loadfn, "glGetUniformfv");
         storage::GetUniformiv.load(&mut loadfn, "glGetUniformiv");
         storage::GetVertexAttribPointerv.load(&mut loadfn, "glGetVertexAttribPointerv");
         storage::GetVertexAttribfv.load(&mut loadfn, "glGetVertexAttribfv");
         storage::GetVertexAttribiv.load(&mut loadfn, "glGetVertexAttribiv");
         storage::Hint.load(&mut loadfn, "glHint");
         storage::IsBuffer.load(&mut loadfn, "glIsBuffer");
         storage::IsEnabled.load(&mut loadfn, "glIsEnabled");
         storage::IsFramebuffer.load(&mut loadfn, "glIsFramebuffer");
         storage::IsProgram.load(&mut loadfn, "glIsProgram");
         storage::IsRenderbuffer.load(&mut loadfn, "glIsRenderbuffer");
         storage::IsShader.load(&mut loadfn, "glIsShader");
         storage::IsTexture.load(&mut loadfn, "glIsTexture");
         storage::LineWidth.load(&mut loadfn, "glLineWidth");
         storage::LinkProgram.load(&mut loadfn, "glLinkProgram");
         storage::PixelStorei.load(&mut loadfn, "glPixelStorei");
         storage::PolygonOffset.load(&mut loadfn, "glPolygonOffset");
         storage::ReadPixels.load(&mut loadfn, "glReadPixels");
         storage::ReleaseShaderCompiler.load(&mut loadfn, "glReleaseShaderCompiler");
         storage::RenderbufferStorage.load(&mut loadfn, "glRenderbufferStorage");
         storage::SampleCoverage.load(&mut loadfn, "glSampleCoverage");
         storage::Scissor.load(&mut loadfn, "glScissor");
         storage::ShaderBinary.load(&mut loadfn, "glShaderBinary");
         storage::ShaderSource.load(&mut loadfn, "glShaderSource");
         storage::StencilFunc.load(&mut loadfn, "glStencilFunc");
         storage::StencilFuncSeparate.load(&mut loadfn, "glStencilFuncSeparate");
         storage::StencilMask.load(&mut loadfn, "glStencilMask");
         storage::StencilMaskSeparate.load(&mut loadfn, "glStencilMaskSeparate");
         storage::StencilOp.load(&mut loadfn, "glStencilOp");
         storage::StencilOpSeparate.load(&mut loadfn, "glStencilOpSeparate");
         storage::TexImage2D.load(&mut loadfn, "glTexImage2D");
         storage::TexParameterf.load(&mut loadfn, "glTexParameterf");
         storage::TexParameterfv.load(&mut loadfn, "glTexParameterfv");
         storage::TexParameteri.load(&mut loadfn, "glTexParameteri");
         storage::TexParameteriv.load(&mut loadfn, "glTexParameteriv");
         storage::TexSubImage2D.load(&mut loadfn, "glTexSubImage2D");
         storage::Uniform1f.load(&mut loadfn, "glUniform1f");
         storage::Uniform1fv.load(&mut loadfn, "glUniform1fv");
         storage::Uniform1i.load(&mut loadfn, "glUniform1i");
         storage::Uniform1iv.load(&mut loadfn, "glUniform1iv");
         storage::Uniform2f.load(&mut loadfn, "glUniform2f");
         storage::Uniform2fv.load(&mut loadfn, "glUniform2fv");
         storage::Uniform2i.load(&mut loadfn, "glUniform2i");
         storage::Uniform2iv.load(&mut loadfn, "glUniform2iv");
         storage::Uniform3f.load(&mut loadfn, "glUniform3f");
         storage::Uniform3fv.load(&mut loadfn, "glUniform3fv");
         storage::Uniform3i.load(&mut loadfn, "glUniform3i");
         storage::Uniform3iv.load(&mut loadfn, "glUniform3iv");
         storage::Uniform4f.load(&mut loadfn, "glUniform4f");
         storage::Uniform4fv.load(&mut loadfn, "glUniform4fv");
         storage::Uniform4i.load(&mut loadfn, "glUniform4i");
         storage::Uniform4iv.load(&mut loadfn, "glUniform4iv");
         storage::UniformMatrix2fv.load(&mut loadfn, "glUniformMatrix2fv");
         storage::UniformMatrix3fv.load(&mut loadfn, "glUniformMatrix3fv");
         storage::UniformMatrix4fv.load(&mut loadfn, "glUniformMatrix4fv");
         storage::UseProgram.load(&mut loadfn, "glUseProgram");
         storage::ValidateProgram.load(&mut loadfn, "glValidateProgram");
         storage::VertexAttrib1f.load(&mut loadfn, "glVertexAttrib1f");
         storage::VertexAttrib1fv.load(&mut loadfn, "glVertexAttrib1fv");
         storage::VertexAttrib2f.load(&mut loadfn, "glVertexAttrib2f");
         storage::VertexAttrib2fv.load(&mut loadfn, "glVertexAttrib2fv");
         storage::VertexAttrib3f.load(&mut loadfn, "glVertexAttrib3f");
         storage::VertexAttrib3fv.load(&mut loadfn, "glVertexAttrib3fv");
         storage::VertexAttrib4f.load(&mut loadfn, "glVertexAttrib4f");
         storage::VertexAttrib4fv.load(&mut loadfn, "glVertexAttrib4fv");
         storage::VertexAttribPointer.load(&mut loadfn, "glVertexAttribPointer");
         storage::Viewport.load(&mut loadfn, "glViewport");

    }
}

