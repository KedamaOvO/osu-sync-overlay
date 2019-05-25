
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
    fn not_initialized() -> ! { panic!("gl: function not initialized") }
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

    pub const GL_2D: std::os::raw::c_uint = 0x0600;
    pub const GL_2_BYTES: std::os::raw::c_uint = 0x1407;
    pub const GL_3D: std::os::raw::c_uint = 0x0601;
    pub const GL_3D_COLOR: std::os::raw::c_uint = 0x0602;
    pub const GL_3D_COLOR_TEXTURE: std::os::raw::c_uint = 0x0603;
    pub const GL_3_BYTES: std::os::raw::c_uint = 0x1408;
    pub const GL_4D_COLOR_TEXTURE: std::os::raw::c_uint = 0x0604;
    pub const GL_4_BYTES: std::os::raw::c_uint = 0x1409;
    pub const GL_ACCUM: std::os::raw::c_uint = 0x0100;
    pub const GL_ACCUM_ALPHA_BITS: std::os::raw::c_uint = 0x0D5B;
    pub const GL_ACCUM_BLUE_BITS: std::os::raw::c_uint = 0x0D5A;
    pub const GL_ACCUM_BUFFER_BIT: std::os::raw::c_uint = 0x00000200;
    pub const GL_ACCUM_CLEAR_VALUE: std::os::raw::c_uint = 0x0B80;
    pub const GL_ACCUM_GREEN_BITS: std::os::raw::c_uint = 0x0D59;
    pub const GL_ACCUM_RED_BITS: std::os::raw::c_uint = 0x0D58;
    pub const GL_ACTIVE_ATTRIBUTES: std::os::raw::c_uint = 0x8B89;
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: std::os::raw::c_uint = 0x8B8A;
    pub const GL_ACTIVE_TEXTURE: std::os::raw::c_uint = 0x84E0;
    pub const GL_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8B86;
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: std::os::raw::c_uint = 0x8B87;
    pub const GL_ADD: std::os::raw::c_uint = 0x0104;
    pub const GL_ADD_SIGNED: std::os::raw::c_uint = 0x8574;
    pub const GL_ALIASED_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x846E;
    pub const GL_ALIASED_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x846D;
    pub const GL_ALL_ATTRIB_BITS: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_ALPHA: std::os::raw::c_uint = 0x1906;
    pub const GL_ALPHA12: std::os::raw::c_uint = 0x803D;
    pub const GL_ALPHA16: std::os::raw::c_uint = 0x803E;
    pub const GL_ALPHA4: std::os::raw::c_uint = 0x803B;
    pub const GL_ALPHA8: std::os::raw::c_uint = 0x803C;
    pub const GL_ALPHA_BIAS: std::os::raw::c_uint = 0x0D1D;
    pub const GL_ALPHA_BITS: std::os::raw::c_uint = 0x0D55;
    pub const GL_ALPHA_SCALE: std::os::raw::c_uint = 0x0D1C;
    pub const GL_ALPHA_TEST: std::os::raw::c_uint = 0x0BC0;
    pub const GL_ALPHA_TEST_FUNC: std::os::raw::c_uint = 0x0BC1;
    pub const GL_ALPHA_TEST_REF: std::os::raw::c_uint = 0x0BC2;
    pub const GL_ALWAYS: std::os::raw::c_uint = 0x0207;
    pub const GL_AMBIENT: std::os::raw::c_uint = 0x1200;
    pub const GL_AMBIENT_AND_DIFFUSE: std::os::raw::c_uint = 0x1602;
    pub const GL_AND: std::os::raw::c_uint = 0x1501;
    pub const GL_AND_INVERTED: std::os::raw::c_uint = 0x1504;
    pub const GL_AND_REVERSE: std::os::raw::c_uint = 0x1502;
    pub const GL_ARRAY_BUFFER: std::os::raw::c_uint = 0x8892;
    pub const GL_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8894;
    pub const GL_ATTACHED_SHADERS: std::os::raw::c_uint = 0x8B85;
    pub const GL_ATTRIB_STACK_DEPTH: std::os::raw::c_uint = 0x0BB0;
    pub const GL_AUTO_NORMAL: std::os::raw::c_uint = 0x0D80;
    pub const GL_AUX0: std::os::raw::c_uint = 0x0409;
    pub const GL_AUX1: std::os::raw::c_uint = 0x040A;
    pub const GL_AUX2: std::os::raw::c_uint = 0x040B;
    pub const GL_AUX3: std::os::raw::c_uint = 0x040C;
    pub const GL_AUX_BUFFERS: std::os::raw::c_uint = 0x0C00;
    pub const GL_BACK: std::os::raw::c_uint = 0x0405;
    pub const GL_BACK_LEFT: std::os::raw::c_uint = 0x0402;
    pub const GL_BACK_RIGHT: std::os::raw::c_uint = 0x0403;
    pub const GL_BGR: std::os::raw::c_uint = 0x80E0;
    pub const GL_BGRA: std::os::raw::c_uint = 0x80E1;
    pub const GL_BITMAP: std::os::raw::c_uint = 0x1A00;
    pub const GL_BITMAP_TOKEN: std::os::raw::c_uint = 0x0704;
    pub const GL_BLEND: std::os::raw::c_uint = 0x0BE2;
    pub const GL_BLEND_COLOR: std::os::raw::c_uint = 0x8005;
    pub const GL_BLEND_DST: std::os::raw::c_uint = 0x0BE0;
    pub const GL_BLEND_DST_ALPHA: std::os::raw::c_uint = 0x80CA;
    pub const GL_BLEND_DST_RGB: std::os::raw::c_uint = 0x80C8;
    pub const GL_BLEND_EQUATION: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_EQUATION_ALPHA: std::os::raw::c_uint = 0x883D;
    pub const GL_BLEND_EQUATION_RGB: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_SRC: std::os::raw::c_uint = 0x0BE1;
    pub const GL_BLEND_SRC_ALPHA: std::os::raw::c_uint = 0x80CB;
    pub const GL_BLEND_SRC_RGB: std::os::raw::c_uint = 0x80C9;
    pub const GL_BLUE: std::os::raw::c_uint = 0x1905;
    pub const GL_BLUE_BIAS: std::os::raw::c_uint = 0x0D1B;
    pub const GL_BLUE_BITS: std::os::raw::c_uint = 0x0D54;
    pub const GL_BLUE_SCALE: std::os::raw::c_uint = 0x0D1A;
    pub const GL_BOOL: std::os::raw::c_uint = 0x8B56;
    pub const GL_BOOL_VEC2: std::os::raw::c_uint = 0x8B57;
    pub const GL_BOOL_VEC3: std::os::raw::c_uint = 0x8B58;
    pub const GL_BOOL_VEC4: std::os::raw::c_uint = 0x8B59;
    pub const GL_BUFFER_ACCESS: std::os::raw::c_uint = 0x88BB;
    pub const GL_BUFFER_MAPPED: std::os::raw::c_uint = 0x88BC;
    pub const GL_BUFFER_MAP_POINTER: std::os::raw::c_uint = 0x88BD;
    pub const GL_BUFFER_SIZE: std::os::raw::c_uint = 0x8764;
    pub const GL_BUFFER_USAGE: std::os::raw::c_uint = 0x8765;
    pub const GL_BYTE: std::os::raw::c_uint = 0x1400;
    pub const GL_C3F_V3F: std::os::raw::c_uint = 0x2A24;
    pub const GL_C4F_N3F_V3F: std::os::raw::c_uint = 0x2A26;
    pub const GL_C4UB_V2F: std::os::raw::c_uint = 0x2A22;
    pub const GL_C4UB_V3F: std::os::raw::c_uint = 0x2A23;
    pub const GL_CCW: std::os::raw::c_uint = 0x0901;
    pub const GL_CLAMP: std::os::raw::c_uint = 0x2900;
    pub const GL_CLAMP_TO_BORDER: std::os::raw::c_uint = 0x812D;
    pub const GL_CLAMP_TO_EDGE: std::os::raw::c_uint = 0x812F;
    pub const GL_CLEAR: std::os::raw::c_uint = 0x1500;
    pub const GL_CLIENT_ACTIVE_TEXTURE: std::os::raw::c_uint = 0x84E1;
    pub const GL_CLIENT_ALL_ATTRIB_BITS: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_CLIENT_ATTRIB_STACK_DEPTH: std::os::raw::c_uint = 0x0BB1;
    pub const GL_CLIENT_PIXEL_STORE_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_CLIENT_VERTEX_ARRAY_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_CLIP_PLANE0: std::os::raw::c_uint = 0x3000;
    pub const GL_CLIP_PLANE1: std::os::raw::c_uint = 0x3001;
    pub const GL_CLIP_PLANE2: std::os::raw::c_uint = 0x3002;
    pub const GL_CLIP_PLANE3: std::os::raw::c_uint = 0x3003;
    pub const GL_CLIP_PLANE4: std::os::raw::c_uint = 0x3004;
    pub const GL_CLIP_PLANE5: std::os::raw::c_uint = 0x3005;
    pub const GL_COEFF: std::os::raw::c_uint = 0x0A00;
    pub const GL_COLOR: std::os::raw::c_uint = 0x1800;
    pub const GL_COLOR_ARRAY: std::os::raw::c_uint = 0x8076;
    pub const GL_COLOR_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8898;
    pub const GL_COLOR_ARRAY_POINTER: std::os::raw::c_uint = 0x8090;
    pub const GL_COLOR_ARRAY_SIZE: std::os::raw::c_uint = 0x8081;
    pub const GL_COLOR_ARRAY_STRIDE: std::os::raw::c_uint = 0x8083;
    pub const GL_COLOR_ARRAY_TYPE: std::os::raw::c_uint = 0x8082;
    pub const GL_COLOR_BUFFER_BIT: std::os::raw::c_uint = 0x00004000;
    pub const GL_COLOR_CLEAR_VALUE: std::os::raw::c_uint = 0x0C22;
    pub const GL_COLOR_INDEX: std::os::raw::c_uint = 0x1900;
    pub const GL_COLOR_INDEXES: std::os::raw::c_uint = 0x1603;
    pub const GL_COLOR_LOGIC_OP: std::os::raw::c_uint = 0x0BF2;
    pub const GL_COLOR_MATERIAL: std::os::raw::c_uint = 0x0B57;
    pub const GL_COLOR_MATERIAL_FACE: std::os::raw::c_uint = 0x0B55;
    pub const GL_COLOR_MATERIAL_PARAMETER: std::os::raw::c_uint = 0x0B56;
    pub const GL_COLOR_SUM: std::os::raw::c_uint = 0x8458;
    pub const GL_COLOR_WRITEMASK: std::os::raw::c_uint = 0x0C23;
    pub const GL_COMBINE: std::os::raw::c_uint = 0x8570;
    pub const GL_COMBINE_ALPHA: std::os::raw::c_uint = 0x8572;
    pub const GL_COMBINE_RGB: std::os::raw::c_uint = 0x8571;
    pub const GL_COMPARE_R_TO_TEXTURE: std::os::raw::c_uint = 0x884E;
    pub const GL_COMPILE: std::os::raw::c_uint = 0x1300;
    pub const GL_COMPILE_AND_EXECUTE: std::os::raw::c_uint = 0x1301;
    pub const GL_COMPILE_STATUS: std::os::raw::c_uint = 0x8B81;
    pub const GL_COMPRESSED_ALPHA: std::os::raw::c_uint = 0x84E9;
    pub const GL_COMPRESSED_INTENSITY: std::os::raw::c_uint = 0x84EC;
    pub const GL_COMPRESSED_LUMINANCE: std::os::raw::c_uint = 0x84EA;
    pub const GL_COMPRESSED_LUMINANCE_ALPHA: std::os::raw::c_uint = 0x84EB;
    pub const GL_COMPRESSED_RGB: std::os::raw::c_uint = 0x84ED;
    pub const GL_COMPRESSED_RGBA: std::os::raw::c_uint = 0x84EE;
    pub const GL_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A3;
    pub const GL_CONSTANT: std::os::raw::c_uint = 0x8576;
    pub const GL_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8003;
    pub const GL_CONSTANT_ATTENUATION: std::os::raw::c_uint = 0x1207;
    pub const GL_CONSTANT_COLOR: std::os::raw::c_uint = 0x8001;
    pub const GL_COORD_REPLACE: std::os::raw::c_uint = 0x8862;
    pub const GL_COPY: std::os::raw::c_uint = 0x1503;
    pub const GL_COPY_INVERTED: std::os::raw::c_uint = 0x150C;
    pub const GL_COPY_PIXEL_TOKEN: std::os::raw::c_uint = 0x0706;
    pub const GL_CULL_FACE: std::os::raw::c_uint = 0x0B44;
    pub const GL_CULL_FACE_MODE: std::os::raw::c_uint = 0x0B45;
    pub const GL_CURRENT_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_CURRENT_COLOR: std::os::raw::c_uint = 0x0B00;
    pub const GL_CURRENT_FOG_COORD: std::os::raw::c_uint = 0x8453;
    pub const GL_CURRENT_FOG_COORDINATE: std::os::raw::c_uint = 0x8453;
    pub const GL_CURRENT_INDEX: std::os::raw::c_uint = 0x0B01;
    pub const GL_CURRENT_NORMAL: std::os::raw::c_uint = 0x0B02;
    pub const GL_CURRENT_PROGRAM: std::os::raw::c_uint = 0x8B8D;
    pub const GL_CURRENT_QUERY: std::os::raw::c_uint = 0x8865;
    pub const GL_CURRENT_RASTER_COLOR: std::os::raw::c_uint = 0x0B04;
    pub const GL_CURRENT_RASTER_DISTANCE: std::os::raw::c_uint = 0x0B09;
    pub const GL_CURRENT_RASTER_INDEX: std::os::raw::c_uint = 0x0B05;
    pub const GL_CURRENT_RASTER_POSITION: std::os::raw::c_uint = 0x0B07;
    pub const GL_CURRENT_RASTER_POSITION_VALID: std::os::raw::c_uint = 0x0B08;
    pub const GL_CURRENT_RASTER_TEXTURE_COORDS: std::os::raw::c_uint = 0x0B06;
    pub const GL_CURRENT_SECONDARY_COLOR: std::os::raw::c_uint = 0x8459;
    pub const GL_CURRENT_TEXTURE_COORDS: std::os::raw::c_uint = 0x0B03;
    pub const GL_CURRENT_VERTEX_ATTRIB: std::os::raw::c_uint = 0x8626;
    pub const GL_CW: std::os::raw::c_uint = 0x0900;
    pub const GL_DECAL: std::os::raw::c_uint = 0x2101;
    pub const GL_DECR: std::os::raw::c_uint = 0x1E03;
    pub const GL_DECR_WRAP: std::os::raw::c_uint = 0x8508;
    pub const GL_DELETE_STATUS: std::os::raw::c_uint = 0x8B80;
    pub const GL_DEPTH: std::os::raw::c_uint = 0x1801;
    pub const GL_DEPTH_BIAS: std::os::raw::c_uint = 0x0D1F;
    pub const GL_DEPTH_BITS: std::os::raw::c_uint = 0x0D56;
    pub const GL_DEPTH_BUFFER_BIT: std::os::raw::c_uint = 0x00000100;
    pub const GL_DEPTH_CLEAR_VALUE: std::os::raw::c_uint = 0x0B73;
    pub const GL_DEPTH_COMPONENT: std::os::raw::c_uint = 0x1902;
    pub const GL_DEPTH_COMPONENT16: std::os::raw::c_uint = 0x81A5;
    pub const GL_DEPTH_COMPONENT24: std::os::raw::c_uint = 0x81A6;
    pub const GL_DEPTH_COMPONENT32: std::os::raw::c_uint = 0x81A7;
    pub const GL_DEPTH_FUNC: std::os::raw::c_uint = 0x0B74;
    pub const GL_DEPTH_RANGE: std::os::raw::c_uint = 0x0B70;
    pub const GL_DEPTH_SCALE: std::os::raw::c_uint = 0x0D1E;
    pub const GL_DEPTH_TEST: std::os::raw::c_uint = 0x0B71;
    pub const GL_DEPTH_TEXTURE_MODE: std::os::raw::c_uint = 0x884B;
    pub const GL_DEPTH_WRITEMASK: std::os::raw::c_uint = 0x0B72;
    pub const GL_DIFFUSE: std::os::raw::c_uint = 0x1201;
    pub const GL_DITHER: std::os::raw::c_uint = 0x0BD0;
    pub const GL_DOMAIN: std::os::raw::c_uint = 0x0A02;
    pub const GL_DONT_CARE: std::os::raw::c_uint = 0x1100;
    pub const GL_DOT3_RGB: std::os::raw::c_uint = 0x86AE;
    pub const GL_DOT3_RGBA: std::os::raw::c_uint = 0x86AF;
    pub const GL_DOUBLE: std::os::raw::c_uint = 0x140A;
    pub const GL_DOUBLEBUFFER: std::os::raw::c_uint = 0x0C32;
    pub const GL_DRAW_BUFFER: std::os::raw::c_uint = 0x0C01;
    pub const GL_DRAW_BUFFER0: std::os::raw::c_uint = 0x8825;
    pub const GL_DRAW_BUFFER1: std::os::raw::c_uint = 0x8826;
    pub const GL_DRAW_BUFFER10: std::os::raw::c_uint = 0x882F;
    pub const GL_DRAW_BUFFER11: std::os::raw::c_uint = 0x8830;
    pub const GL_DRAW_BUFFER12: std::os::raw::c_uint = 0x8831;
    pub const GL_DRAW_BUFFER13: std::os::raw::c_uint = 0x8832;
    pub const GL_DRAW_BUFFER14: std::os::raw::c_uint = 0x8833;
    pub const GL_DRAW_BUFFER15: std::os::raw::c_uint = 0x8834;
    pub const GL_DRAW_BUFFER2: std::os::raw::c_uint = 0x8827;
    pub const GL_DRAW_BUFFER3: std::os::raw::c_uint = 0x8828;
    pub const GL_DRAW_BUFFER4: std::os::raw::c_uint = 0x8829;
    pub const GL_DRAW_BUFFER5: std::os::raw::c_uint = 0x882A;
    pub const GL_DRAW_BUFFER6: std::os::raw::c_uint = 0x882B;
    pub const GL_DRAW_BUFFER7: std::os::raw::c_uint = 0x882C;
    pub const GL_DRAW_BUFFER8: std::os::raw::c_uint = 0x882D;
    pub const GL_DRAW_BUFFER9: std::os::raw::c_uint = 0x882E;
    pub const GL_DRAW_PIXEL_TOKEN: std::os::raw::c_uint = 0x0705;
    pub const GL_DST_ALPHA: std::os::raw::c_uint = 0x0304;
    pub const GL_DST_COLOR: std::os::raw::c_uint = 0x0306;
    pub const GL_DYNAMIC_COPY: std::os::raw::c_uint = 0x88EA;
    pub const GL_DYNAMIC_DRAW: std::os::raw::c_uint = 0x88E8;
    pub const GL_DYNAMIC_READ: std::os::raw::c_uint = 0x88E9;
    pub const GL_EDGE_FLAG: std::os::raw::c_uint = 0x0B43;
    pub const GL_EDGE_FLAG_ARRAY: std::os::raw::c_uint = 0x8079;
    pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889B;
    pub const GL_EDGE_FLAG_ARRAY_POINTER: std::os::raw::c_uint = 0x8093;
    pub const GL_EDGE_FLAG_ARRAY_STRIDE: std::os::raw::c_uint = 0x808C;
    pub const GL_ELEMENT_ARRAY_BUFFER: std::os::raw::c_uint = 0x8893;
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8895;
    pub const GL_EMISSION: std::os::raw::c_uint = 0x1600;
    pub const GL_ENABLE_BIT: std::os::raw::c_uint = 0x00002000;
    pub const GL_EQUAL: std::os::raw::c_uint = 0x0202;
    pub const GL_EQUIV: std::os::raw::c_uint = 0x1509;
    pub const GL_EVAL_BIT: std::os::raw::c_uint = 0x00010000;
    pub const GL_EXP: std::os::raw::c_uint = 0x0800;
    pub const GL_EXP2: std::os::raw::c_uint = 0x0801;
    pub const GL_EXTENSIONS: std::os::raw::c_uint = 0x1F03;
    pub const GL_EYE_LINEAR: std::os::raw::c_uint = 0x2400;
    pub const GL_EYE_PLANE: std::os::raw::c_uint = 0x2502;
    pub const GL_FALSE: std::os::raw::c_uchar = 0;
    pub const GL_FASTEST: std::os::raw::c_uint = 0x1101;
    pub const GL_FEEDBACK: std::os::raw::c_uint = 0x1C01;
    pub const GL_FEEDBACK_BUFFER_POINTER: std::os::raw::c_uint = 0x0DF0;
    pub const GL_FEEDBACK_BUFFER_SIZE: std::os::raw::c_uint = 0x0DF1;
    pub const GL_FEEDBACK_BUFFER_TYPE: std::os::raw::c_uint = 0x0DF2;
    pub const GL_FILL: std::os::raw::c_uint = 0x1B02;
    pub const GL_FLAT: std::os::raw::c_uint = 0x1D00;
    pub const GL_FLOAT: std::os::raw::c_uint = 0x1406;
    pub const GL_FLOAT_MAT2: std::os::raw::c_uint = 0x8B5A;
    pub const GL_FLOAT_MAT3: std::os::raw::c_uint = 0x8B5B;
    pub const GL_FLOAT_MAT4: std::os::raw::c_uint = 0x8B5C;
    pub const GL_FLOAT_VEC2: std::os::raw::c_uint = 0x8B50;
    pub const GL_FLOAT_VEC3: std::os::raw::c_uint = 0x8B51;
    pub const GL_FLOAT_VEC4: std::os::raw::c_uint = 0x8B52;
    pub const GL_FOG: std::os::raw::c_uint = 0x0B60;
    pub const GL_FOG_BIT: std::os::raw::c_uint = 0x00000080;
    pub const GL_FOG_COLOR: std::os::raw::c_uint = 0x0B66;
    pub const GL_FOG_COORD: std::os::raw::c_uint = 0x8451;
    pub const GL_FOG_COORDINATE: std::os::raw::c_uint = 0x8451;
    pub const GL_FOG_COORDINATE_ARRAY: std::os::raw::c_uint = 0x8457;
    pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889D;
    pub const GL_FOG_COORDINATE_ARRAY_POINTER: std::os::raw::c_uint = 0x8456;
    pub const GL_FOG_COORDINATE_ARRAY_STRIDE: std::os::raw::c_uint = 0x8455;
    pub const GL_FOG_COORDINATE_ARRAY_TYPE: std::os::raw::c_uint = 0x8454;
    pub const GL_FOG_COORDINATE_SOURCE: std::os::raw::c_uint = 0x8450;
    pub const GL_FOG_COORD_ARRAY: std::os::raw::c_uint = 0x8457;
    pub const GL_FOG_COORD_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889D;
    pub const GL_FOG_COORD_ARRAY_POINTER: std::os::raw::c_uint = 0x8456;
    pub const GL_FOG_COORD_ARRAY_STRIDE: std::os::raw::c_uint = 0x8455;
    pub const GL_FOG_COORD_ARRAY_TYPE: std::os::raw::c_uint = 0x8454;
    pub const GL_FOG_COORD_SRC: std::os::raw::c_uint = 0x8450;
    pub const GL_FOG_DENSITY: std::os::raw::c_uint = 0x0B62;
    pub const GL_FOG_END: std::os::raw::c_uint = 0x0B64;
    pub const GL_FOG_HINT: std::os::raw::c_uint = 0x0C54;
    pub const GL_FOG_INDEX: std::os::raw::c_uint = 0x0B61;
    pub const GL_FOG_MODE: std::os::raw::c_uint = 0x0B65;
    pub const GL_FOG_START: std::os::raw::c_uint = 0x0B63;
    pub const GL_FRAGMENT_DEPTH: std::os::raw::c_uint = 0x8452;
    pub const GL_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8B30;
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: std::os::raw::c_uint = 0x8B8B;
    pub const GL_FRONT: std::os::raw::c_uint = 0x0404;
    pub const GL_FRONT_AND_BACK: std::os::raw::c_uint = 0x0408;
    pub const GL_FRONT_FACE: std::os::raw::c_uint = 0x0B46;
    pub const GL_FRONT_LEFT: std::os::raw::c_uint = 0x0400;
    pub const GL_FRONT_RIGHT: std::os::raw::c_uint = 0x0401;
    pub const GL_FUNC_ADD: std::os::raw::c_uint = 0x8006;
    pub const GL_FUNC_REVERSE_SUBTRACT: std::os::raw::c_uint = 0x800B;
    pub const GL_FUNC_SUBTRACT: std::os::raw::c_uint = 0x800A;
    pub const GL_GENERATE_MIPMAP: std::os::raw::c_uint = 0x8191;
    pub const GL_GENERATE_MIPMAP_HINT: std::os::raw::c_uint = 0x8192;
    pub const GL_GEQUAL: std::os::raw::c_uint = 0x0206;
    pub const GL_GREATER: std::os::raw::c_uint = 0x0204;
    pub const GL_GREEN: std::os::raw::c_uint = 0x1904;
    pub const GL_GREEN_BIAS: std::os::raw::c_uint = 0x0D19;
    pub const GL_GREEN_BITS: std::os::raw::c_uint = 0x0D53;
    pub const GL_GREEN_SCALE: std::os::raw::c_uint = 0x0D18;
    pub const GL_HINT_BIT: std::os::raw::c_uint = 0x00008000;
    pub const GL_INCR: std::os::raw::c_uint = 0x1E02;
    pub const GL_INCR_WRAP: std::os::raw::c_uint = 0x8507;
    pub const GL_INDEX_ARRAY: std::os::raw::c_uint = 0x8077;
    pub const GL_INDEX_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8899;
    pub const GL_INDEX_ARRAY_POINTER: std::os::raw::c_uint = 0x8091;
    pub const GL_INDEX_ARRAY_STRIDE: std::os::raw::c_uint = 0x8086;
    pub const GL_INDEX_ARRAY_TYPE: std::os::raw::c_uint = 0x8085;
    pub const GL_INDEX_BITS: std::os::raw::c_uint = 0x0D51;
    pub const GL_INDEX_CLEAR_VALUE: std::os::raw::c_uint = 0x0C20;
    pub const GL_INDEX_LOGIC_OP: std::os::raw::c_uint = 0x0BF1;
    pub const GL_INDEX_MODE: std::os::raw::c_uint = 0x0C30;
    pub const GL_INDEX_OFFSET: std::os::raw::c_uint = 0x0D13;
    pub const GL_INDEX_SHIFT: std::os::raw::c_uint = 0x0D12;
    pub const GL_INDEX_WRITEMASK: std::os::raw::c_uint = 0x0C21;
    pub const GL_INFO_LOG_LENGTH: std::os::raw::c_uint = 0x8B84;
    pub const GL_INT: std::os::raw::c_uint = 0x1404;
    pub const GL_INTENSITY: std::os::raw::c_uint = 0x8049;
    pub const GL_INTENSITY12: std::os::raw::c_uint = 0x804C;
    pub const GL_INTENSITY16: std::os::raw::c_uint = 0x804D;
    pub const GL_INTENSITY4: std::os::raw::c_uint = 0x804A;
    pub const GL_INTENSITY8: std::os::raw::c_uint = 0x804B;
    pub const GL_INTERPOLATE: std::os::raw::c_uint = 0x8575;
    pub const GL_INT_VEC2: std::os::raw::c_uint = 0x8B53;
    pub const GL_INT_VEC3: std::os::raw::c_uint = 0x8B54;
    pub const GL_INT_VEC4: std::os::raw::c_uint = 0x8B55;
    pub const GL_INVALID_ENUM: std::os::raw::c_uint = 0x0500;
    pub const GL_INVALID_OPERATION: std::os::raw::c_uint = 0x0502;
    pub const GL_INVALID_VALUE: std::os::raw::c_uint = 0x0501;
    pub const GL_INVERT: std::os::raw::c_uint = 0x150A;
    pub const GL_KEEP: std::os::raw::c_uint = 0x1E00;
    pub const GL_LEFT: std::os::raw::c_uint = 0x0406;
    pub const GL_LEQUAL: std::os::raw::c_uint = 0x0203;
    pub const GL_LESS: std::os::raw::c_uint = 0x0201;
    pub const GL_LIGHT0: std::os::raw::c_uint = 0x4000;
    pub const GL_LIGHT1: std::os::raw::c_uint = 0x4001;
    pub const GL_LIGHT2: std::os::raw::c_uint = 0x4002;
    pub const GL_LIGHT3: std::os::raw::c_uint = 0x4003;
    pub const GL_LIGHT4: std::os::raw::c_uint = 0x4004;
    pub const GL_LIGHT5: std::os::raw::c_uint = 0x4005;
    pub const GL_LIGHT6: std::os::raw::c_uint = 0x4006;
    pub const GL_LIGHT7: std::os::raw::c_uint = 0x4007;
    pub const GL_LIGHTING: std::os::raw::c_uint = 0x0B50;
    pub const GL_LIGHTING_BIT: std::os::raw::c_uint = 0x00000040;
    pub const GL_LIGHT_MODEL_AMBIENT: std::os::raw::c_uint = 0x0B53;
    pub const GL_LIGHT_MODEL_COLOR_CONTROL: std::os::raw::c_uint = 0x81F8;
    pub const GL_LIGHT_MODEL_LOCAL_VIEWER: std::os::raw::c_uint = 0x0B51;
    pub const GL_LIGHT_MODEL_TWO_SIDE: std::os::raw::c_uint = 0x0B52;
    pub const GL_LINE: std::os::raw::c_uint = 0x1B01;
    pub const GL_LINEAR: std::os::raw::c_uint = 0x2601;
    pub const GL_LINEAR_ATTENUATION: std::os::raw::c_uint = 0x1208;
    pub const GL_LINEAR_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2703;
    pub const GL_LINEAR_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2701;
    pub const GL_LINES: std::os::raw::c_uint = 0x0001;
    pub const GL_LINE_BIT: std::os::raw::c_uint = 0x00000004;
    pub const GL_LINE_LOOP: std::os::raw::c_uint = 0x0002;
    pub const GL_LINE_RESET_TOKEN: std::os::raw::c_uint = 0x0707;
    pub const GL_LINE_SMOOTH: std::os::raw::c_uint = 0x0B20;
    pub const GL_LINE_SMOOTH_HINT: std::os::raw::c_uint = 0x0C52;
    pub const GL_LINE_STIPPLE: std::os::raw::c_uint = 0x0B24;
    pub const GL_LINE_STIPPLE_PATTERN: std::os::raw::c_uint = 0x0B25;
    pub const GL_LINE_STIPPLE_REPEAT: std::os::raw::c_uint = 0x0B26;
    pub const GL_LINE_STRIP: std::os::raw::c_uint = 0x0003;
    pub const GL_LINE_TOKEN: std::os::raw::c_uint = 0x0702;
    pub const GL_LINE_WIDTH: std::os::raw::c_uint = 0x0B21;
    pub const GL_LINE_WIDTH_GRANULARITY: std::os::raw::c_uint = 0x0B23;
    pub const GL_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x0B22;
    pub const GL_LINK_STATUS: std::os::raw::c_uint = 0x8B82;
    pub const GL_LIST_BASE: std::os::raw::c_uint = 0x0B32;
    pub const GL_LIST_BIT: std::os::raw::c_uint = 0x00020000;
    pub const GL_LIST_INDEX: std::os::raw::c_uint = 0x0B33;
    pub const GL_LIST_MODE: std::os::raw::c_uint = 0x0B30;
    pub const GL_LOAD: std::os::raw::c_uint = 0x0101;
    pub const GL_LOGIC_OP: std::os::raw::c_uint = 0x0BF1;
    pub const GL_LOGIC_OP_MODE: std::os::raw::c_uint = 0x0BF0;
    pub const GL_LOWER_LEFT: std::os::raw::c_uint = 0x8CA1;
    pub const GL_LUMINANCE: std::os::raw::c_uint = 0x1909;
    pub const GL_LUMINANCE12: std::os::raw::c_uint = 0x8041;
    pub const GL_LUMINANCE12_ALPHA12: std::os::raw::c_uint = 0x8047;
    pub const GL_LUMINANCE12_ALPHA4: std::os::raw::c_uint = 0x8046;
    pub const GL_LUMINANCE16: std::os::raw::c_uint = 0x8042;
    pub const GL_LUMINANCE16_ALPHA16: std::os::raw::c_uint = 0x8048;
    pub const GL_LUMINANCE4: std::os::raw::c_uint = 0x803F;
    pub const GL_LUMINANCE4_ALPHA4: std::os::raw::c_uint = 0x8043;
    pub const GL_LUMINANCE6_ALPHA2: std::os::raw::c_uint = 0x8044;
    pub const GL_LUMINANCE8: std::os::raw::c_uint = 0x8040;
    pub const GL_LUMINANCE8_ALPHA8: std::os::raw::c_uint = 0x8045;
    pub const GL_LUMINANCE_ALPHA: std::os::raw::c_uint = 0x190A;
    pub const GL_MAP1_COLOR_4: std::os::raw::c_uint = 0x0D90;
    pub const GL_MAP1_GRID_DOMAIN: std::os::raw::c_uint = 0x0DD0;
    pub const GL_MAP1_GRID_SEGMENTS: std::os::raw::c_uint = 0x0DD1;
    pub const GL_MAP1_INDEX: std::os::raw::c_uint = 0x0D91;
    pub const GL_MAP1_NORMAL: std::os::raw::c_uint = 0x0D92;
    pub const GL_MAP1_TEXTURE_COORD_1: std::os::raw::c_uint = 0x0D93;
    pub const GL_MAP1_TEXTURE_COORD_2: std::os::raw::c_uint = 0x0D94;
    pub const GL_MAP1_TEXTURE_COORD_3: std::os::raw::c_uint = 0x0D95;
    pub const GL_MAP1_TEXTURE_COORD_4: std::os::raw::c_uint = 0x0D96;
    pub const GL_MAP1_VERTEX_3: std::os::raw::c_uint = 0x0D97;
    pub const GL_MAP1_VERTEX_4: std::os::raw::c_uint = 0x0D98;
    pub const GL_MAP2_COLOR_4: std::os::raw::c_uint = 0x0DB0;
    pub const GL_MAP2_GRID_DOMAIN: std::os::raw::c_uint = 0x0DD2;
    pub const GL_MAP2_GRID_SEGMENTS: std::os::raw::c_uint = 0x0DD3;
    pub const GL_MAP2_INDEX: std::os::raw::c_uint = 0x0DB1;
    pub const GL_MAP2_NORMAL: std::os::raw::c_uint = 0x0DB2;
    pub const GL_MAP2_TEXTURE_COORD_1: std::os::raw::c_uint = 0x0DB3;
    pub const GL_MAP2_TEXTURE_COORD_2: std::os::raw::c_uint = 0x0DB4;
    pub const GL_MAP2_TEXTURE_COORD_3: std::os::raw::c_uint = 0x0DB5;
    pub const GL_MAP2_TEXTURE_COORD_4: std::os::raw::c_uint = 0x0DB6;
    pub const GL_MAP2_VERTEX_3: std::os::raw::c_uint = 0x0DB7;
    pub const GL_MAP2_VERTEX_4: std::os::raw::c_uint = 0x0DB8;
    pub const GL_MAP_COLOR: std::os::raw::c_uint = 0x0D10;
    pub const GL_MAP_STENCIL: std::os::raw::c_uint = 0x0D11;
    pub const GL_MATRIX_MODE: std::os::raw::c_uint = 0x0BA0;
    pub const GL_MAX: std::os::raw::c_uint = 0x8008;
    pub const GL_MAX_3D_TEXTURE_SIZE: std::os::raw::c_uint = 0x8073;
    pub const GL_MAX_ATTRIB_STACK_DEPTH: std::os::raw::c_uint = 0x0D35;
    pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: std::os::raw::c_uint = 0x0D3B;
    pub const GL_MAX_CLIP_PLANES: std::os::raw::c_uint = 0x0D32;
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4D;
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: std::os::raw::c_uint = 0x851C;
    pub const GL_MAX_DRAW_BUFFERS: std::os::raw::c_uint = 0x8824;
    pub const GL_MAX_ELEMENTS_INDICES: std::os::raw::c_uint = 0x80E9;
    pub const GL_MAX_ELEMENTS_VERTICES: std::os::raw::c_uint = 0x80E8;
    pub const GL_MAX_EVAL_ORDER: std::os::raw::c_uint = 0x0D30;
    pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B49;
    pub const GL_MAX_LIGHTS: std::os::raw::c_uint = 0x0D31;
    pub const GL_MAX_LIST_NESTING: std::os::raw::c_uint = 0x0B31;
    pub const GL_MAX_MODELVIEW_STACK_DEPTH: std::os::raw::c_uint = 0x0D36;
    pub const GL_MAX_NAME_STACK_DEPTH: std::os::raw::c_uint = 0x0D37;
    pub const GL_MAX_PIXEL_MAP_TABLE: std::os::raw::c_uint = 0x0D34;
    pub const GL_MAX_PROJECTION_STACK_DEPTH: std::os::raw::c_uint = 0x0D38;
    pub const GL_MAX_TEXTURE_COORDS: std::os::raw::c_uint = 0x8871;
    pub const GL_MAX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8872;
    pub const GL_MAX_TEXTURE_LOD_BIAS: std::os::raw::c_uint = 0x84FD;
    pub const GL_MAX_TEXTURE_SIZE: std::os::raw::c_uint = 0x0D33;
    pub const GL_MAX_TEXTURE_STACK_DEPTH: std::os::raw::c_uint = 0x0D39;
    pub const GL_MAX_TEXTURE_UNITS: std::os::raw::c_uint = 0x84E2;
    pub const GL_MAX_VARYING_FLOATS: std::os::raw::c_uint = 0x8B4B;
    pub const GL_MAX_VERTEX_ATTRIBS: std::os::raw::c_uint = 0x8869;
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4C;
    pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B4A;
    pub const GL_MAX_VIEWPORT_DIMS: std::os::raw::c_uint = 0x0D3A;
    pub const GL_MIN: std::os::raw::c_uint = 0x8007;
    pub const GL_MIRRORED_REPEAT: std::os::raw::c_uint = 0x8370;
    pub const GL_MODELVIEW: std::os::raw::c_uint = 0x1700;
    pub const GL_MODELVIEW_MATRIX: std::os::raw::c_uint = 0x0BA6;
    pub const GL_MODELVIEW_STACK_DEPTH: std::os::raw::c_uint = 0x0BA3;
    pub const GL_MODULATE: std::os::raw::c_uint = 0x2100;
    pub const GL_MULT: std::os::raw::c_uint = 0x0103;
    pub const GL_MULTISAMPLE: std::os::raw::c_uint = 0x809D;
    pub const GL_MULTISAMPLE_BIT: std::os::raw::c_uint = 0x20000000;
    pub const GL_N3F_V3F: std::os::raw::c_uint = 0x2A25;
    pub const GL_NAME_STACK_DEPTH: std::os::raw::c_uint = 0x0D70;
    pub const GL_NAND: std::os::raw::c_uint = 0x150E;
    pub const GL_NEAREST: std::os::raw::c_uint = 0x2600;
    pub const GL_NEAREST_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2702;
    pub const GL_NEAREST_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2700;
    pub const GL_NEVER: std::os::raw::c_uint = 0x0200;
    pub const GL_NICEST: std::os::raw::c_uint = 0x1102;
    pub const GL_NONE: std::os::raw::c_uint = 0;
    pub const GL_NOOP: std::os::raw::c_uint = 0x1505;
    pub const GL_NOR: std::os::raw::c_uint = 0x1508;
    pub const GL_NORMALIZE: std::os::raw::c_uint = 0x0BA1;
    pub const GL_NORMAL_ARRAY: std::os::raw::c_uint = 0x8075;
    pub const GL_NORMAL_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8897;
    pub const GL_NORMAL_ARRAY_POINTER: std::os::raw::c_uint = 0x808F;
    pub const GL_NORMAL_ARRAY_STRIDE: std::os::raw::c_uint = 0x807F;
    pub const GL_NORMAL_ARRAY_TYPE: std::os::raw::c_uint = 0x807E;
    pub const GL_NORMAL_MAP: std::os::raw::c_uint = 0x8511;
    pub const GL_NOTEQUAL: std::os::raw::c_uint = 0x0205;
    pub const GL_NO_ERROR: std::os::raw::c_uint = 0;
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A2;
    pub const GL_OBJECT_LINEAR: std::os::raw::c_uint = 0x2401;
    pub const GL_OBJECT_PLANE: std::os::raw::c_uint = 0x2501;
    pub const GL_ONE: std::os::raw::c_uint = 1;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8004;
    pub const GL_ONE_MINUS_CONSTANT_COLOR: std::os::raw::c_uint = 0x8002;
    pub const GL_ONE_MINUS_DST_ALPHA: std::os::raw::c_uint = 0x0305;
    pub const GL_ONE_MINUS_DST_COLOR: std::os::raw::c_uint = 0x0307;
    pub const GL_ONE_MINUS_SRC_ALPHA: std::os::raw::c_uint = 0x0303;
    pub const GL_ONE_MINUS_SRC_COLOR: std::os::raw::c_uint = 0x0301;
    pub const GL_OPERAND0_ALPHA: std::os::raw::c_uint = 0x8598;
    pub const GL_OPERAND0_RGB: std::os::raw::c_uint = 0x8590;
    pub const GL_OPERAND1_ALPHA: std::os::raw::c_uint = 0x8599;
    pub const GL_OPERAND1_RGB: std::os::raw::c_uint = 0x8591;
    pub const GL_OPERAND2_ALPHA: std::os::raw::c_uint = 0x859A;
    pub const GL_OPERAND2_RGB: std::os::raw::c_uint = 0x8592;
    pub const GL_OR: std::os::raw::c_uint = 0x1507;
    pub const GL_ORDER: std::os::raw::c_uint = 0x0A01;
    pub const GL_OR_INVERTED: std::os::raw::c_uint = 0x150D;
    pub const GL_OR_REVERSE: std::os::raw::c_uint = 0x150B;
    pub const GL_OUT_OF_MEMORY: std::os::raw::c_uint = 0x0505;
    pub const GL_PACK_ALIGNMENT: std::os::raw::c_uint = 0x0D05;
    pub const GL_PACK_IMAGE_HEIGHT: std::os::raw::c_uint = 0x806C;
    pub const GL_PACK_LSB_FIRST: std::os::raw::c_uint = 0x0D01;
    pub const GL_PACK_ROW_LENGTH: std::os::raw::c_uint = 0x0D02;
    pub const GL_PACK_SKIP_IMAGES: std::os::raw::c_uint = 0x806B;
    pub const GL_PACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0D04;
    pub const GL_PACK_SKIP_ROWS: std::os::raw::c_uint = 0x0D03;
    pub const GL_PACK_SWAP_BYTES: std::os::raw::c_uint = 0x0D00;
    pub const GL_PASS_THROUGH_TOKEN: std::os::raw::c_uint = 0x0700;
    pub const GL_PERSPECTIVE_CORRECTION_HINT: std::os::raw::c_uint = 0x0C50;
    pub const GL_PIXEL_MAP_A_TO_A: std::os::raw::c_uint = 0x0C79;
    pub const GL_PIXEL_MAP_A_TO_A_SIZE: std::os::raw::c_uint = 0x0CB9;
    pub const GL_PIXEL_MAP_B_TO_B: std::os::raw::c_uint = 0x0C78;
    pub const GL_PIXEL_MAP_B_TO_B_SIZE: std::os::raw::c_uint = 0x0CB8;
    pub const GL_PIXEL_MAP_G_TO_G: std::os::raw::c_uint = 0x0C77;
    pub const GL_PIXEL_MAP_G_TO_G_SIZE: std::os::raw::c_uint = 0x0CB7;
    pub const GL_PIXEL_MAP_I_TO_A: std::os::raw::c_uint = 0x0C75;
    pub const GL_PIXEL_MAP_I_TO_A_SIZE: std::os::raw::c_uint = 0x0CB5;
    pub const GL_PIXEL_MAP_I_TO_B: std::os::raw::c_uint = 0x0C74;
    pub const GL_PIXEL_MAP_I_TO_B_SIZE: std::os::raw::c_uint = 0x0CB4;
    pub const GL_PIXEL_MAP_I_TO_G: std::os::raw::c_uint = 0x0C73;
    pub const GL_PIXEL_MAP_I_TO_G_SIZE: std::os::raw::c_uint = 0x0CB3;
    pub const GL_PIXEL_MAP_I_TO_I: std::os::raw::c_uint = 0x0C70;
    pub const GL_PIXEL_MAP_I_TO_I_SIZE: std::os::raw::c_uint = 0x0CB0;
    pub const GL_PIXEL_MAP_I_TO_R: std::os::raw::c_uint = 0x0C72;
    pub const GL_PIXEL_MAP_I_TO_R_SIZE: std::os::raw::c_uint = 0x0CB2;
    pub const GL_PIXEL_MAP_R_TO_R: std::os::raw::c_uint = 0x0C76;
    pub const GL_PIXEL_MAP_R_TO_R_SIZE: std::os::raw::c_uint = 0x0CB6;
    pub const GL_PIXEL_MAP_S_TO_S: std::os::raw::c_uint = 0x0C71;
    pub const GL_PIXEL_MAP_S_TO_S_SIZE: std::os::raw::c_uint = 0x0CB1;
    pub const GL_PIXEL_MODE_BIT: std::os::raw::c_uint = 0x00000020;
    pub const GL_POINT: std::os::raw::c_uint = 0x1B00;
    pub const GL_POINTS: std::os::raw::c_uint = 0x0000;
    pub const GL_POINT_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_POINT_DISTANCE_ATTENUATION: std::os::raw::c_uint = 0x8129;
    pub const GL_POINT_FADE_THRESHOLD_SIZE: std::os::raw::c_uint = 0x8128;
    pub const GL_POINT_SIZE: std::os::raw::c_uint = 0x0B11;
    pub const GL_POINT_SIZE_GRANULARITY: std::os::raw::c_uint = 0x0B13;
    pub const GL_POINT_SIZE_MAX: std::os::raw::c_uint = 0x8127;
    pub const GL_POINT_SIZE_MIN: std::os::raw::c_uint = 0x8126;
    pub const GL_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x0B12;
    pub const GL_POINT_SMOOTH: std::os::raw::c_uint = 0x0B10;
    pub const GL_POINT_SMOOTH_HINT: std::os::raw::c_uint = 0x0C51;
    pub const GL_POINT_SPRITE: std::os::raw::c_uint = 0x8861;
    pub const GL_POINT_SPRITE_COORD_ORIGIN: std::os::raw::c_uint = 0x8CA0;
    pub const GL_POINT_TOKEN: std::os::raw::c_uint = 0x0701;
    pub const GL_POLYGON: std::os::raw::c_uint = 0x0009;
    pub const GL_POLYGON_BIT: std::os::raw::c_uint = 0x00000008;
    pub const GL_POLYGON_MODE: std::os::raw::c_uint = 0x0B40;
    pub const GL_POLYGON_OFFSET_FACTOR: std::os::raw::c_uint = 0x8038;
    pub const GL_POLYGON_OFFSET_FILL: std::os::raw::c_uint = 0x8037;
    pub const GL_POLYGON_OFFSET_LINE: std::os::raw::c_uint = 0x2A02;
    pub const GL_POLYGON_OFFSET_POINT: std::os::raw::c_uint = 0x2A01;
    pub const GL_POLYGON_OFFSET_UNITS: std::os::raw::c_uint = 0x2A00;
    pub const GL_POLYGON_SMOOTH: std::os::raw::c_uint = 0x0B41;
    pub const GL_POLYGON_SMOOTH_HINT: std::os::raw::c_uint = 0x0C53;
    pub const GL_POLYGON_STIPPLE: std::os::raw::c_uint = 0x0B42;
    pub const GL_POLYGON_STIPPLE_BIT: std::os::raw::c_uint = 0x00000010;
    pub const GL_POLYGON_TOKEN: std::os::raw::c_uint = 0x0703;
    pub const GL_POSITION: std::os::raw::c_uint = 0x1203;
    pub const GL_PREVIOUS: std::os::raw::c_uint = 0x8578;
    pub const GL_PRIMARY_COLOR: std::os::raw::c_uint = 0x8577;
    pub const GL_PROJECTION: std::os::raw::c_uint = 0x1701;
    pub const GL_PROJECTION_MATRIX: std::os::raw::c_uint = 0x0BA7;
    pub const GL_PROJECTION_STACK_DEPTH: std::os::raw::c_uint = 0x0BA4;
    pub const GL_PROXY_TEXTURE_1D: std::os::raw::c_uint = 0x8063;
    pub const GL_PROXY_TEXTURE_2D: std::os::raw::c_uint = 0x8064;
    pub const GL_PROXY_TEXTURE_3D: std::os::raw::c_uint = 0x8070;
    pub const GL_PROXY_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x851B;
    pub const GL_Q: std::os::raw::c_uint = 0x2003;
    pub const GL_QUADRATIC_ATTENUATION: std::os::raw::c_uint = 0x1209;
    pub const GL_QUADS: std::os::raw::c_uint = 0x0007;
    pub const GL_QUAD_STRIP: std::os::raw::c_uint = 0x0008;
    pub const GL_QUERY_COUNTER_BITS: std::os::raw::c_uint = 0x8864;
    pub const GL_QUERY_RESULT: std::os::raw::c_uint = 0x8866;
    pub const GL_QUERY_RESULT_AVAILABLE: std::os::raw::c_uint = 0x8867;
    pub const GL_R: std::os::raw::c_uint = 0x2002;
    pub const GL_R3_G3_B2: std::os::raw::c_uint = 0x2A10;
    pub const GL_READ_BUFFER: std::os::raw::c_uint = 0x0C02;
    pub const GL_READ_ONLY: std::os::raw::c_uint = 0x88B8;
    pub const GL_READ_WRITE: std::os::raw::c_uint = 0x88BA;
    pub const GL_RED: std::os::raw::c_uint = 0x1903;
    pub const GL_RED_BIAS: std::os::raw::c_uint = 0x0D15;
    pub const GL_RED_BITS: std::os::raw::c_uint = 0x0D52;
    pub const GL_RED_SCALE: std::os::raw::c_uint = 0x0D14;
    pub const GL_REFLECTION_MAP: std::os::raw::c_uint = 0x8512;
    pub const GL_RENDER: std::os::raw::c_uint = 0x1C00;
    pub const GL_RENDERER: std::os::raw::c_uint = 0x1F01;
    pub const GL_RENDER_MODE: std::os::raw::c_uint = 0x0C40;
    pub const GL_REPEAT: std::os::raw::c_uint = 0x2901;
    pub const GL_REPLACE: std::os::raw::c_uint = 0x1E01;
    pub const GL_RESCALE_NORMAL: std::os::raw::c_uint = 0x803A;
    pub const GL_RETURN: std::os::raw::c_uint = 0x0102;
    pub const GL_RGB: std::os::raw::c_uint = 0x1907;
    pub const GL_RGB10: std::os::raw::c_uint = 0x8052;
    pub const GL_RGB10_A2: std::os::raw::c_uint = 0x8059;
    pub const GL_RGB12: std::os::raw::c_uint = 0x8053;
    pub const GL_RGB16: std::os::raw::c_uint = 0x8054;
    pub const GL_RGB4: std::os::raw::c_uint = 0x804F;
    pub const GL_RGB5: std::os::raw::c_uint = 0x8050;
    pub const GL_RGB5_A1: std::os::raw::c_uint = 0x8057;
    pub const GL_RGB8: std::os::raw::c_uint = 0x8051;
    pub const GL_RGBA: std::os::raw::c_uint = 0x1908;
    pub const GL_RGBA12: std::os::raw::c_uint = 0x805A;
    pub const GL_RGBA16: std::os::raw::c_uint = 0x805B;
    pub const GL_RGBA2: std::os::raw::c_uint = 0x8055;
    pub const GL_RGBA4: std::os::raw::c_uint = 0x8056;
    pub const GL_RGBA8: std::os::raw::c_uint = 0x8058;
    pub const GL_RGBA_MODE: std::os::raw::c_uint = 0x0C31;
    pub const GL_RGB_SCALE: std::os::raw::c_uint = 0x8573;
    pub const GL_RIGHT: std::os::raw::c_uint = 0x0407;
    pub const GL_S: std::os::raw::c_uint = 0x2000;
    pub const GL_SAMPLER_1D: std::os::raw::c_uint = 0x8B5D;
    pub const GL_SAMPLER_1D_SHADOW: std::os::raw::c_uint = 0x8B61;
    pub const GL_SAMPLER_2D: std::os::raw::c_uint = 0x8B5E;
    pub const GL_SAMPLER_2D_SHADOW: std::os::raw::c_uint = 0x8B62;
    pub const GL_SAMPLER_3D: std::os::raw::c_uint = 0x8B5F;
    pub const GL_SAMPLER_CUBE: std::os::raw::c_uint = 0x8B60;
    pub const GL_SAMPLES: std::os::raw::c_uint = 0x80A9;
    pub const GL_SAMPLES_PASSED: std::os::raw::c_uint = 0x8914;
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE: std::os::raw::c_uint = 0x809E;
    pub const GL_SAMPLE_ALPHA_TO_ONE: std::os::raw::c_uint = 0x809F;
    pub const GL_SAMPLE_BUFFERS: std::os::raw::c_uint = 0x80A8;
    pub const GL_SAMPLE_COVERAGE: std::os::raw::c_uint = 0x80A0;
    pub const GL_SAMPLE_COVERAGE_INVERT: std::os::raw::c_uint = 0x80AB;
    pub const GL_SAMPLE_COVERAGE_VALUE: std::os::raw::c_uint = 0x80AA;
    pub const GL_SCISSOR_BIT: std::os::raw::c_uint = 0x00080000;
    pub const GL_SCISSOR_BOX: std::os::raw::c_uint = 0x0C10;
    pub const GL_SCISSOR_TEST: std::os::raw::c_uint = 0x0C11;
    pub const GL_SECONDARY_COLOR_ARRAY: std::os::raw::c_uint = 0x845E;
    pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889C;
    pub const GL_SECONDARY_COLOR_ARRAY_POINTER: std::os::raw::c_uint = 0x845D;
    pub const GL_SECONDARY_COLOR_ARRAY_SIZE: std::os::raw::c_uint = 0x845A;
    pub const GL_SECONDARY_COLOR_ARRAY_STRIDE: std::os::raw::c_uint = 0x845C;
    pub const GL_SECONDARY_COLOR_ARRAY_TYPE: std::os::raw::c_uint = 0x845B;
    pub const GL_SELECT: std::os::raw::c_uint = 0x1C02;
    pub const GL_SELECTION_BUFFER_POINTER: std::os::raw::c_uint = 0x0DF3;
    pub const GL_SELECTION_BUFFER_SIZE: std::os::raw::c_uint = 0x0DF4;
    pub const GL_SEPARATE_SPECULAR_COLOR: std::os::raw::c_uint = 0x81FA;
    pub const GL_SET: std::os::raw::c_uint = 0x150F;
    pub const GL_SHADER_SOURCE_LENGTH: std::os::raw::c_uint = 0x8B88;
    pub const GL_SHADER_TYPE: std::os::raw::c_uint = 0x8B4F;
    pub const GL_SHADE_MODEL: std::os::raw::c_uint = 0x0B54;
    pub const GL_SHADING_LANGUAGE_VERSION: std::os::raw::c_uint = 0x8B8C;
    pub const GL_SHININESS: std::os::raw::c_uint = 0x1601;
    pub const GL_SHORT: std::os::raw::c_uint = 0x1402;
    pub const GL_SINGLE_COLOR: std::os::raw::c_uint = 0x81F9;
    pub const GL_SMOOTH: std::os::raw::c_uint = 0x1D01;
    pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: std::os::raw::c_uint = 0x0B23;
    pub const GL_SMOOTH_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x0B22;
    pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: std::os::raw::c_uint = 0x0B13;
    pub const GL_SMOOTH_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x0B12;
    pub const GL_SOURCE0_ALPHA: std::os::raw::c_uint = 0x8588;
    pub const GL_SOURCE0_RGB: std::os::raw::c_uint = 0x8580;
    pub const GL_SOURCE1_ALPHA: std::os::raw::c_uint = 0x8589;
    pub const GL_SOURCE1_RGB: std::os::raw::c_uint = 0x8581;
    pub const GL_SOURCE2_ALPHA: std::os::raw::c_uint = 0x858A;
    pub const GL_SOURCE2_RGB: std::os::raw::c_uint = 0x8582;
    pub const GL_SPECULAR: std::os::raw::c_uint = 0x1202;
    pub const GL_SPHERE_MAP: std::os::raw::c_uint = 0x2402;
    pub const GL_SPOT_CUTOFF: std::os::raw::c_uint = 0x1206;
    pub const GL_SPOT_DIRECTION: std::os::raw::c_uint = 0x1204;
    pub const GL_SPOT_EXPONENT: std::os::raw::c_uint = 0x1205;
    pub const GL_SRC0_ALPHA: std::os::raw::c_uint = 0x8588;
    pub const GL_SRC0_RGB: std::os::raw::c_uint = 0x8580;
    pub const GL_SRC1_ALPHA: std::os::raw::c_uint = 0x8589;
    pub const GL_SRC1_RGB: std::os::raw::c_uint = 0x8581;
    pub const GL_SRC2_ALPHA: std::os::raw::c_uint = 0x858A;
    pub const GL_SRC2_RGB: std::os::raw::c_uint = 0x8582;
    pub const GL_SRC_ALPHA: std::os::raw::c_uint = 0x0302;
    pub const GL_SRC_ALPHA_SATURATE: std::os::raw::c_uint = 0x0308;
    pub const GL_SRC_COLOR: std::os::raw::c_uint = 0x0300;
    pub const GL_STACK_OVERFLOW: std::os::raw::c_uint = 0x0503;
    pub const GL_STACK_UNDERFLOW: std::os::raw::c_uint = 0x0504;
    pub const GL_STATIC_COPY: std::os::raw::c_uint = 0x88E6;
    pub const GL_STATIC_DRAW: std::os::raw::c_uint = 0x88E4;
    pub const GL_STATIC_READ: std::os::raw::c_uint = 0x88E5;
    pub const GL_STENCIL: std::os::raw::c_uint = 0x1802;
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
    pub const GL_STENCIL_INDEX: std::os::raw::c_uint = 0x1901;
    pub const GL_STENCIL_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x0B96;
    pub const GL_STENCIL_REF: std::os::raw::c_uint = 0x0B97;
    pub const GL_STENCIL_TEST: std::os::raw::c_uint = 0x0B90;
    pub const GL_STENCIL_VALUE_MASK: std::os::raw::c_uint = 0x0B93;
    pub const GL_STENCIL_WRITEMASK: std::os::raw::c_uint = 0x0B98;
    pub const GL_STEREO: std::os::raw::c_uint = 0x0C33;
    pub const GL_STREAM_COPY: std::os::raw::c_uint = 0x88E2;
    pub const GL_STREAM_DRAW: std::os::raw::c_uint = 0x88E0;
    pub const GL_STREAM_READ: std::os::raw::c_uint = 0x88E1;
    pub const GL_SUBPIXEL_BITS: std::os::raw::c_uint = 0x0D50;
    pub const GL_SUBTRACT: std::os::raw::c_uint = 0x84E7;
    pub const GL_T: std::os::raw::c_uint = 0x2001;
    pub const GL_T2F_C3F_V3F: std::os::raw::c_uint = 0x2A2A;
    pub const GL_T2F_C4F_N3F_V3F: std::os::raw::c_uint = 0x2A2C;
    pub const GL_T2F_C4UB_V3F: std::os::raw::c_uint = 0x2A29;
    pub const GL_T2F_N3F_V3F: std::os::raw::c_uint = 0x2A2B;
    pub const GL_T2F_V3F: std::os::raw::c_uint = 0x2A27;
    pub const GL_T4F_C4F_N3F_V4F: std::os::raw::c_uint = 0x2A2D;
    pub const GL_T4F_V4F: std::os::raw::c_uint = 0x2A28;
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
    pub const GL_TEXTURE_1D: std::os::raw::c_uint = 0x0DE0;
    pub const GL_TEXTURE_2D: std::os::raw::c_uint = 0x0DE1;
    pub const GL_TEXTURE_3D: std::os::raw::c_uint = 0x806F;
    pub const GL_TEXTURE_ALPHA_SIZE: std::os::raw::c_uint = 0x805F;
    pub const GL_TEXTURE_BASE_LEVEL: std::os::raw::c_uint = 0x813C;
    pub const GL_TEXTURE_BINDING_1D: std::os::raw::c_uint = 0x8068;
    pub const GL_TEXTURE_BINDING_2D: std::os::raw::c_uint = 0x8069;
    pub const GL_TEXTURE_BINDING_3D: std::os::raw::c_uint = 0x806A;
    pub const GL_TEXTURE_BINDING_CUBE_MAP: std::os::raw::c_uint = 0x8514;
    pub const GL_TEXTURE_BIT: std::os::raw::c_uint = 0x00040000;
    pub const GL_TEXTURE_BLUE_SIZE: std::os::raw::c_uint = 0x805E;
    pub const GL_TEXTURE_BORDER: std::os::raw::c_uint = 0x1005;
    pub const GL_TEXTURE_BORDER_COLOR: std::os::raw::c_uint = 0x1004;
    pub const GL_TEXTURE_COMPARE_FUNC: std::os::raw::c_uint = 0x884D;
    pub const GL_TEXTURE_COMPARE_MODE: std::os::raw::c_uint = 0x884C;
    pub const GL_TEXTURE_COMPONENTS: std::os::raw::c_uint = 0x1003;
    pub const GL_TEXTURE_COMPRESSED: std::os::raw::c_uint = 0x86A1;
    pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: std::os::raw::c_uint = 0x86A0;
    pub const GL_TEXTURE_COMPRESSION_HINT: std::os::raw::c_uint = 0x84EF;
    pub const GL_TEXTURE_COORD_ARRAY: std::os::raw::c_uint = 0x8078;
    pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889A;
    pub const GL_TEXTURE_COORD_ARRAY_POINTER: std::os::raw::c_uint = 0x8092;
    pub const GL_TEXTURE_COORD_ARRAY_SIZE: std::os::raw::c_uint = 0x8088;
    pub const GL_TEXTURE_COORD_ARRAY_STRIDE: std::os::raw::c_uint = 0x808A;
    pub const GL_TEXTURE_COORD_ARRAY_TYPE: std::os::raw::c_uint = 0x8089;
    pub const GL_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x8513;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: std::os::raw::c_uint = 0x8516;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: std::os::raw::c_uint = 0x8518;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: std::os::raw::c_uint = 0x851A;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: std::os::raw::c_uint = 0x8515;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: std::os::raw::c_uint = 0x8517;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: std::os::raw::c_uint = 0x8519;
    pub const GL_TEXTURE_DEPTH: std::os::raw::c_uint = 0x8071;
    pub const GL_TEXTURE_DEPTH_SIZE: std::os::raw::c_uint = 0x884A;
    pub const GL_TEXTURE_ENV: std::os::raw::c_uint = 0x2300;
    pub const GL_TEXTURE_ENV_COLOR: std::os::raw::c_uint = 0x2201;
    pub const GL_TEXTURE_ENV_MODE: std::os::raw::c_uint = 0x2200;
    pub const GL_TEXTURE_FILTER_CONTROL: std::os::raw::c_uint = 0x8500;
    pub const GL_TEXTURE_GEN_MODE: std::os::raw::c_uint = 0x2500;
    pub const GL_TEXTURE_GEN_Q: std::os::raw::c_uint = 0x0C63;
    pub const GL_TEXTURE_GEN_R: std::os::raw::c_uint = 0x0C62;
    pub const GL_TEXTURE_GEN_S: std::os::raw::c_uint = 0x0C60;
    pub const GL_TEXTURE_GEN_T: std::os::raw::c_uint = 0x0C61;
    pub const GL_TEXTURE_GREEN_SIZE: std::os::raw::c_uint = 0x805D;
    pub const GL_TEXTURE_HEIGHT: std::os::raw::c_uint = 0x1001;
    pub const GL_TEXTURE_INTENSITY_SIZE: std::os::raw::c_uint = 0x8061;
    pub const GL_TEXTURE_INTERNAL_FORMAT: std::os::raw::c_uint = 0x1003;
    pub const GL_TEXTURE_LOD_BIAS: std::os::raw::c_uint = 0x8501;
    pub const GL_TEXTURE_LUMINANCE_SIZE: std::os::raw::c_uint = 0x8060;
    pub const GL_TEXTURE_MAG_FILTER: std::os::raw::c_uint = 0x2800;
    pub const GL_TEXTURE_MATRIX: std::os::raw::c_uint = 0x0BA8;
    pub const GL_TEXTURE_MAX_LEVEL: std::os::raw::c_uint = 0x813D;
    pub const GL_TEXTURE_MAX_LOD: std::os::raw::c_uint = 0x813B;
    pub const GL_TEXTURE_MIN_FILTER: std::os::raw::c_uint = 0x2801;
    pub const GL_TEXTURE_MIN_LOD: std::os::raw::c_uint = 0x813A;
    pub const GL_TEXTURE_PRIORITY: std::os::raw::c_uint = 0x8066;
    pub const GL_TEXTURE_RED_SIZE: std::os::raw::c_uint = 0x805C;
    pub const GL_TEXTURE_RESIDENT: std::os::raw::c_uint = 0x8067;
    pub const GL_TEXTURE_STACK_DEPTH: std::os::raw::c_uint = 0x0BA5;
    pub const GL_TEXTURE_WIDTH: std::os::raw::c_uint = 0x1000;
    pub const GL_TEXTURE_WRAP_R: std::os::raw::c_uint = 0x8072;
    pub const GL_TEXTURE_WRAP_S: std::os::raw::c_uint = 0x2802;
    pub const GL_TEXTURE_WRAP_T: std::os::raw::c_uint = 0x2803;
    pub const GL_TRANSFORM_BIT: std::os::raw::c_uint = 0x00001000;
    pub const GL_TRANSPOSE_COLOR_MATRIX: std::os::raw::c_uint = 0x84E6;
    pub const GL_TRANSPOSE_MODELVIEW_MATRIX: std::os::raw::c_uint = 0x84E3;
    pub const GL_TRANSPOSE_PROJECTION_MATRIX: std::os::raw::c_uint = 0x84E4;
    pub const GL_TRANSPOSE_TEXTURE_MATRIX: std::os::raw::c_uint = 0x84E5;
    pub const GL_TRIANGLES: std::os::raw::c_uint = 0x0004;
    pub const GL_TRIANGLE_FAN: std::os::raw::c_uint = 0x0006;
    pub const GL_TRIANGLE_STRIP: std::os::raw::c_uint = 0x0005;
    pub const GL_TRUE: std::os::raw::c_uchar = 1;
    pub const GL_UNPACK_ALIGNMENT: std::os::raw::c_uint = 0x0CF5;
    pub const GL_UNPACK_IMAGE_HEIGHT: std::os::raw::c_uint = 0x806E;
    pub const GL_UNPACK_LSB_FIRST: std::os::raw::c_uint = 0x0CF1;
    pub const GL_UNPACK_ROW_LENGTH: std::os::raw::c_uint = 0x0CF2;
    pub const GL_UNPACK_SKIP_IMAGES: std::os::raw::c_uint = 0x806D;
    pub const GL_UNPACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0CF4;
    pub const GL_UNPACK_SKIP_ROWS: std::os::raw::c_uint = 0x0CF3;
    pub const GL_UNPACK_SWAP_BYTES: std::os::raw::c_uint = 0x0CF0;
    pub const GL_UNSIGNED_BYTE: std::os::raw::c_uint = 0x1401;
    pub const GL_UNSIGNED_BYTE_2_3_3_REV: std::os::raw::c_uint = 0x8362;
    pub const GL_UNSIGNED_BYTE_3_3_2: std::os::raw::c_uint = 0x8032;
    pub const GL_UNSIGNED_INT: std::os::raw::c_uint = 0x1405;
    pub const GL_UNSIGNED_INT_10_10_10_2: std::os::raw::c_uint = 0x8036;
    pub const GL_UNSIGNED_INT_2_10_10_10_REV: std::os::raw::c_uint = 0x8368;
    pub const GL_UNSIGNED_INT_8_8_8_8: std::os::raw::c_uint = 0x8035;
    pub const GL_UNSIGNED_INT_8_8_8_8_REV: std::os::raw::c_uint = 0x8367;
    pub const GL_UNSIGNED_SHORT: std::os::raw::c_uint = 0x1403;
    pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: std::os::raw::c_uint = 0x8366;
    pub const GL_UNSIGNED_SHORT_4_4_4_4: std::os::raw::c_uint = 0x8033;
    pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: std::os::raw::c_uint = 0x8365;
    pub const GL_UNSIGNED_SHORT_5_5_5_1: std::os::raw::c_uint = 0x8034;
    pub const GL_UNSIGNED_SHORT_5_6_5: std::os::raw::c_uint = 0x8363;
    pub const GL_UNSIGNED_SHORT_5_6_5_REV: std::os::raw::c_uint = 0x8364;
    pub const GL_UPPER_LEFT: std::os::raw::c_uint = 0x8CA2;
    pub const GL_V2F: std::os::raw::c_uint = 0x2A20;
    pub const GL_V3F: std::os::raw::c_uint = 0x2A21;
    pub const GL_VALIDATE_STATUS: std::os::raw::c_uint = 0x8B83;
    pub const GL_VENDOR: std::os::raw::c_uint = 0x1F00;
    pub const GL_VERSION: std::os::raw::c_uint = 0x1F02;
    pub const GL_VERTEX_ARRAY: std::os::raw::c_uint = 0x8074;
    pub const GL_VERTEX_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8896;
    pub const GL_VERTEX_ARRAY_POINTER: std::os::raw::c_uint = 0x808E;
    pub const GL_VERTEX_ARRAY_SIZE: std::os::raw::c_uint = 0x807A;
    pub const GL_VERTEX_ARRAY_STRIDE: std::os::raw::c_uint = 0x807C;
    pub const GL_VERTEX_ARRAY_TYPE: std::os::raw::c_uint = 0x807B;
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889F;
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: std::os::raw::c_uint = 0x8622;
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: std::os::raw::c_uint = 0x886A;
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: std::os::raw::c_uint = 0x8645;
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: std::os::raw::c_uint = 0x8623;
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: std::os::raw::c_uint = 0x8624;
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: std::os::raw::c_uint = 0x8625;
    pub const GL_VERTEX_PROGRAM_POINT_SIZE: std::os::raw::c_uint = 0x8642;
    pub const GL_VERTEX_PROGRAM_TWO_SIDE: std::os::raw::c_uint = 0x8643;
    pub const GL_VERTEX_SHADER: std::os::raw::c_uint = 0x8B31;
    pub const GL_VIEWPORT: std::os::raw::c_uint = 0x0BA2;
    pub const GL_VIEWPORT_BIT: std::os::raw::c_uint = 0x00000800;
    pub const GL_WEIGHT_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889E;
    pub const GL_WRITE_ONLY: std::os::raw::c_uint = 0x88B9;
    pub const GL_XOR: std::os::raw::c_uint = 0x1506;
    pub const GL_ZERO: std::os::raw::c_uint = 0;
    pub const GL_ZOOM_X: std::os::raw::c_uint = 0x0D16;
    pub const GL_ZOOM_Y: std::os::raw::c_uint = 0x0D17;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code)]

    use std;
    use std::mem;
    use super::storage;
    use super::types::*;

     #[inline] pub unsafe fn Accum(op: GLenum, value: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::Accum.ptr)(op, value) }
     #[inline] pub unsafe fn ActiveTexture(texture: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ActiveTexture.ptr)(texture) }
     #[inline] pub unsafe fn AlphaFunc(func: GLenum, ref_: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::AlphaFunc.ptr)(func, ref_) }
     #[inline] pub unsafe fn AreTexturesResident(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint, *mut GLboolean) -> GLboolean>(storage::AreTexturesResident.ptr)(n, textures, residences) }
     #[inline] pub unsafe fn ArrayElement(i: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ArrayElement.ptr)(i) }
     #[inline] pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::AttachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn Begin(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Begin.ptr)(mode) }
     #[inline] pub unsafe fn BeginQuery(target: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BeginQuery.ptr)(target, id) }
     #[inline] pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindAttribLocation.ptr)(program, index, name) }
     #[inline] pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindBuffer.ptr)(target, buffer) }
     #[inline] pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTexture.ptr)(target, texture) }
     #[inline] pub unsafe fn Bitmap(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLsizei, GLfloat, GLfloat, GLfloat, GLfloat, *const GLubyte) -> ()>(storage::Bitmap.ptr)(width, height, xorig, yorig, xmove, ymove, bitmap) }
     #[inline] pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::BlendColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn BlendEquation(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BlendEquation.ptr)(mode) }
     #[inline] pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendEquationSeparate.ptr)(modeRGB, modeAlpha) }
     #[inline] pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendFunc.ptr)(sfactor, dfactor) }
     #[inline] pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::BlendFuncSeparate.ptr)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
     #[inline] pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const std::os::raw::c_void, usage: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const std::os::raw::c_void, GLenum) -> ()>(storage::BufferData.ptr)(target, size, data, usage) }
     #[inline] pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const std::os::raw::c_void) -> ()>(storage::BufferSubData.ptr)(target, offset, size, data) }
     #[inline] pub unsafe fn CallList(list: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CallList.ptr)(list) }
     #[inline] pub unsafe fn CallLists(n: GLsizei, type_: GLenum, lists: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::CallLists.ptr)(n, type_, lists) }
     #[inline] pub unsafe fn Clear(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::Clear.ptr)(mask) }
     #[inline] pub unsafe fn ClearAccum(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearAccum.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ClearDepth(depth: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::ClearDepth.ptr)(depth) }
     #[inline] pub unsafe fn ClearIndex(c: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::ClearIndex.ptr)(c) }
     #[inline] pub unsafe fn ClearStencil(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ClearStencil.ptr)(s) }
     #[inline] pub unsafe fn ClientActiveTexture(texture: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ClientActiveTexture.ptr)(texture) }
     #[inline] pub unsafe fn ClipPlane(plane: GLenum, equation: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLdouble) -> ()>(storage::ClipPlane.ptr)(plane, equation) }
     #[inline] pub unsafe fn Color3b(red: GLbyte, green: GLbyte, blue: GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLbyte, GLbyte, GLbyte) -> ()>(storage::Color3b.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3bv(v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(*const GLbyte) -> ()>(storage::Color3bv.ptr)(v) }
     #[inline] pub unsafe fn Color3d(red: GLdouble, green: GLdouble, blue: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Color3d.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Color3dv.ptr)(v) }
     #[inline] pub unsafe fn Color3f(red: GLfloat, green: GLfloat, blue: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Color3f.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Color3fv.ptr)(v) }
     #[inline] pub unsafe fn Color3i(red: GLint, green: GLint, blue: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Color3i.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Color3iv.ptr)(v) }
     #[inline] pub unsafe fn Color3s(red: GLshort, green: GLshort, blue: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::Color3s.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Color3sv.ptr)(v) }
     #[inline] pub unsafe fn Color3ub(red: GLubyte, green: GLubyte, blue: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLubyte, GLubyte, GLubyte) -> ()>(storage::Color3ub.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3ubv(v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::Color3ubv.ptr)(v) }
     #[inline] pub unsafe fn Color3ui(red: GLuint, green: GLuint, blue: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::Color3ui.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3uiv(v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(*const GLuint) -> ()>(storage::Color3uiv.ptr)(v) }
     #[inline] pub unsafe fn Color3us(red: GLushort, green: GLushort, blue: GLushort) -> () { mem::transmute::<_, extern "system" fn(GLushort, GLushort, GLushort) -> ()>(storage::Color3us.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3usv(v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(*const GLushort) -> ()>(storage::Color3usv.ptr)(v) }
     #[inline] pub unsafe fn Color4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLbyte, GLbyte, GLbyte, GLbyte) -> ()>(storage::Color4b.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4bv(v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(*const GLbyte) -> ()>(storage::Color4bv.ptr)(v) }
     #[inline] pub unsafe fn Color4d(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Color4d.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Color4dv.ptr)(v) }
     #[inline] pub unsafe fn Color4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Color4f.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Color4fv.ptr)(v) }
     #[inline] pub unsafe fn Color4i(red: GLint, green: GLint, blue: GLint, alpha: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Color4i.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Color4iv.ptr)(v) }
     #[inline] pub unsafe fn Color4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::Color4s.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Color4sv.ptr)(v) }
     #[inline] pub unsafe fn Color4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLubyte, GLubyte, GLubyte, GLubyte) -> ()>(storage::Color4ub.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4ubv(v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::Color4ubv.ptr)(v) }
     #[inline] pub unsafe fn Color4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> ()>(storage::Color4ui.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4uiv(v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(*const GLuint) -> ()>(storage::Color4uiv.ptr)(v) }
     #[inline] pub unsafe fn Color4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort) -> () { mem::transmute::<_, extern "system" fn(GLushort, GLushort, GLushort, GLushort) -> ()>(storage::Color4us.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4usv(v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(*const GLushort) -> ()>(storage::Color4usv.ptr)(v) }
     #[inline] pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMask.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ColorMaterial(face: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::ColorMaterial.ptr)(face, mode) }
     #[inline] pub unsafe fn ColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::ColorPointer.ptr)(size, type_, stride, pointer) }
     #[inline] pub unsafe fn CompileShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CompileShader.ptr)(shader) }
     #[inline] pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage1D.ptr)(target, level, internalformat, width, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage2D.ptr)(target, level, internalformat, width, height, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage3D.ptr)(target, level, internalformat, width, height, depth, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage1D.ptr)(target, level, xoffset, width, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
     #[inline] pub unsafe fn CopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum) -> ()>(storage::CopyPixels.ptr)(x, y, width, height, type_) }
     #[inline] pub unsafe fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint) -> ()>(storage::CopyTexImage1D.ptr)(target, level, internalformat, x, y, width, border) }
     #[inline] pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> ()>(storage::CopyTexImage2D.ptr)(target, level, internalformat, x, y, width, height, border) }
     #[inline] pub unsafe fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei) -> ()>(storage::CopyTexSubImage1D.ptr)(target, level, xoffset, x, y, width) }
     #[inline] pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage2D.ptr)(target, level, xoffset, yoffset, x, y, width, height) }
     #[inline] pub unsafe fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
     #[inline] pub unsafe fn CreateProgram() -> GLuint { mem::transmute::<_, extern "system" fn() -> GLuint>(storage::CreateProgram.ptr)() }
     #[inline] pub unsafe fn CreateShader(type_: GLenum) -> GLuint { mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(storage::CreateShader.ptr)(type_) }
     #[inline] pub unsafe fn CullFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::CullFace.ptr)(mode) }
     #[inline] pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn DeleteLists(list: GLuint, range: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei) -> ()>(storage::DeleteLists.ptr)(list, range) }
     #[inline] pub unsafe fn DeleteProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteProgram.ptr)(program) }
     #[inline] pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteQueries.ptr)(n, ids) }
     #[inline] pub unsafe fn DeleteShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteShader.ptr)(shader) }
     #[inline] pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn DepthFunc(func: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DepthFunc.ptr)(func) }
     #[inline] pub unsafe fn DepthMask(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::DepthMask.ptr)(flag) }
     #[inline] pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::DepthRange.ptr)(n, f) }
     #[inline] pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DetachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn Disable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Disable.ptr)(cap) }
     #[inline] pub unsafe fn DisableClientState(array: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DisableClientState.ptr)(array) }
     #[inline] pub unsafe fn DisableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DisableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(storage::DrawArrays.ptr)(mode, first, count) }
     #[inline] pub unsafe fn DrawBuffer(buf: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DrawBuffer.ptr)(buf) }
     #[inline] pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(storage::DrawBuffers.ptr)(n, bufs) }
     #[inline] pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawElements.ptr)(mode, count, type_, indices) }
     #[inline] pub unsafe fn DrawPixels(width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawPixels.ptr)(width, height, format, type_, pixels) }
     #[inline] pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawRangeElements.ptr)(mode, start, end, count, type_, indices) }
     #[inline] pub unsafe fn EdgeFlag(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::EdgeFlag.ptr)(flag) }
     #[inline] pub unsafe fn EdgeFlagPointer(stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const std::os::raw::c_void) -> ()>(storage::EdgeFlagPointer.ptr)(stride, pointer) }
     #[inline] pub unsafe fn EdgeFlagv(flag: *const GLboolean) -> () { mem::transmute::<_, extern "system" fn(*const GLboolean) -> ()>(storage::EdgeFlagv.ptr)(flag) }
     #[inline] pub unsafe fn Enable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Enable.ptr)(cap) }
     #[inline] pub unsafe fn EnableClientState(array: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::EnableClientState.ptr)(array) }
     #[inline] pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::EnableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn End() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::End.ptr)() }
     #[inline] pub unsafe fn EndList() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndList.ptr)() }
     #[inline] pub unsafe fn EndQuery(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::EndQuery.ptr)(target) }
     #[inline] pub unsafe fn EvalCoord1d(u: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::EvalCoord1d.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord1dv(u: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::EvalCoord1dv.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord1f(u: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::EvalCoord1f.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord1fv(u: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::EvalCoord1fv.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord2d(u: GLdouble, v: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::EvalCoord2d.ptr)(u, v) }
     #[inline] pub unsafe fn EvalCoord2dv(u: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::EvalCoord2dv.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord2f(u: GLfloat, v: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::EvalCoord2f.ptr)(u, v) }
     #[inline] pub unsafe fn EvalCoord2fv(u: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::EvalCoord2fv.ptr)(u) }
     #[inline] pub unsafe fn EvalMesh1(mode: GLenum, i1: GLint, i2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint) -> ()>(storage::EvalMesh1.ptr)(mode, i1, i2) }
     #[inline] pub unsafe fn EvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint) -> ()>(storage::EvalMesh2.ptr)(mode, i1, i2, j1, j2) }
     #[inline] pub unsafe fn EvalPoint1(i: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::EvalPoint1.ptr)(i) }
     #[inline] pub unsafe fn EvalPoint2(i: GLint, j: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::EvalPoint2.ptr)(i, j) }
     #[inline] pub unsafe fn FeedbackBuffer(size: GLsizei, type_: GLenum, buffer: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLenum, *mut GLfloat) -> ()>(storage::FeedbackBuffer.ptr)(size, type_, buffer) }
     #[inline] pub unsafe fn Finish() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.ptr)() }
     #[inline] pub unsafe fn Flush() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.ptr)() }
     #[inline] pub unsafe fn FogCoordPointer(type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::FogCoordPointer.ptr)(type_, stride, pointer) }
     #[inline] pub unsafe fn FogCoordd(coord: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::FogCoordd.ptr)(coord) }
     #[inline] pub unsafe fn FogCoorddv(coord: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::FogCoorddv.ptr)(coord) }
     #[inline] pub unsafe fn FogCoordf(coord: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::FogCoordf.ptr)(coord) }
     #[inline] pub unsafe fn FogCoordfv(coord: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::FogCoordfv.ptr)(coord) }
     #[inline] pub unsafe fn Fogf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::Fogf.ptr)(pname, param) }
     #[inline] pub unsafe fn Fogfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::Fogfv.ptr)(pname, params) }
     #[inline] pub unsafe fn Fogi(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::Fogi.ptr)(pname, param) }
     #[inline] pub unsafe fn Fogiv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::Fogiv.ptr)(pname, params) }
     #[inline] pub unsafe fn FrontFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::FrontFace.ptr)(mode) }
     #[inline] pub unsafe fn Frustum(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Frustum.ptr)(left, right, bottom, top, zNear, zFar) }
     #[inline] pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn GenLists(range: GLsizei) -> GLuint { mem::transmute::<_, extern "system" fn(GLsizei) -> GLuint>(storage::GenLists.ptr)(range) }
     #[inline] pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenQueries.ptr)(n, ids) }
     #[inline] pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveAttrib.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveUniform.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>(storage::GetAttachedShaders.ptr)(program, maxCount, count, shaders) }
     #[inline] pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetAttribLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(storage::GetBooleanv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetBufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetBufferPointerv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut std::os::raw::c_void) -> ()>(storage::GetBufferSubData.ptr)(target, offset, size, data) }
     #[inline] pub unsafe fn GetClipPlane(plane: GLenum, equation: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(storage::GetClipPlane.ptr)(plane, equation) }
     #[inline] pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut std::os::raw::c_void) -> ()>(storage::GetCompressedTexImage.ptr)(target, level, img) }
     #[inline] pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(storage::GetDoublev.ptr)(pname, data) }
     #[inline] pub unsafe fn GetError() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetError.ptr)() }
     #[inline] pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetFloatv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(storage::GetIntegerv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetLightfv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn GetLightiv(light: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetLightiv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn GetMapdv(target: GLenum, query: GLenum, v: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLdouble) -> ()>(storage::GetMapdv.ptr)(target, query, v) }
     #[inline] pub unsafe fn GetMapfv(target: GLenum, query: GLenum, v: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetMapfv.ptr)(target, query, v) }
     #[inline] pub unsafe fn GetMapiv(target: GLenum, query: GLenum, v: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetMapiv.ptr)(target, query, v) }
     #[inline] pub unsafe fn GetMaterialfv(face: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetMaterialfv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn GetMaterialiv(face: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetMaterialiv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn GetPixelMapfv(map: GLenum, values: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetPixelMapfv.ptr)(map, values) }
     #[inline] pub unsafe fn GetPixelMapuiv(map: GLenum, values: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLuint) -> ()>(storage::GetPixelMapuiv.ptr)(map, values) }
     #[inline] pub unsafe fn GetPixelMapusv(map: GLenum, values: *mut GLushort) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLushort) -> ()>(storage::GetPixelMapusv.ptr)(map, values) }
     #[inline] pub unsafe fn GetPointerv(pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetPointerv.ptr)(pname, params) }
     #[inline] pub unsafe fn GetPolygonStipple(mask: *mut GLubyte) -> () { mem::transmute::<_, extern "system" fn(*mut GLubyte) -> ()>(storage::GetPolygonStipple.ptr)(mask) }
     #[inline] pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramInfoLog.ptr)(program, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetProgramiv.ptr)(program, pname, params) }
     #[inline] pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetQueryObjectiv.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetQueryObjectuiv.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetQueryiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderInfoLog.ptr)(shader, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderSource.ptr)(shader, bufSize, length, source) }
     #[inline] pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetShaderiv.ptr)(shader, pname, params) }
     #[inline] pub unsafe fn GetString(name: GLenum) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(storage::GetString.ptr)(name) }
     #[inline] pub unsafe fn GetTexEnvfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexEnvfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexEnviv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexEnviv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexGendv(coord: GLenum, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLdouble) -> ()>(storage::GetTexGendv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn GetTexGenfv(coord: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexGenfv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn GetTexGeniv(coord: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexGeniv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::GetTexImage.ptr)(target, level, format, type_, pixels) }
     #[inline] pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> ()>(storage::GetTexLevelParameterfv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(storage::GetTexLevelParameteriv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetUniformLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>(storage::GetUniformfv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>(storage::GetUniformiv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetVertexAttribPointerv.ptr)(index, pname, pointer) }
     #[inline] pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(storage::GetVertexAttribdv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetVertexAttribfv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::Hint.ptr)(target, mode) }
     #[inline] pub unsafe fn IndexMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::IndexMask.ptr)(mask) }
     #[inline] pub unsafe fn IndexPointer(type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::IndexPointer.ptr)(type_, stride, pointer) }
     #[inline] pub unsafe fn Indexd(c: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::Indexd.ptr)(c) }
     #[inline] pub unsafe fn Indexdv(c: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Indexdv.ptr)(c) }
     #[inline] pub unsafe fn Indexf(c: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::Indexf.ptr)(c) }
     #[inline] pub unsafe fn Indexfv(c: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Indexfv.ptr)(c) }
     #[inline] pub unsafe fn Indexi(c: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::Indexi.ptr)(c) }
     #[inline] pub unsafe fn Indexiv(c: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Indexiv.ptr)(c) }
     #[inline] pub unsafe fn Indexs(c: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort) -> ()>(storage::Indexs.ptr)(c) }
     #[inline] pub unsafe fn Indexsv(c: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Indexsv.ptr)(c) }
     #[inline] pub unsafe fn Indexub(c: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLubyte) -> ()>(storage::Indexub.ptr)(c) }
     #[inline] pub unsafe fn Indexubv(c: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::Indexubv.ptr)(c) }
     #[inline] pub unsafe fn InitNames() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::InitNames.ptr)() }
     #[inline] pub unsafe fn InterleavedArrays(format: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::InterleavedArrays.ptr)(format, stride, pointer) }
     #[inline] pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsBuffer.ptr)(buffer) }
     #[inline] pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::IsEnabled.ptr)(cap) }
     #[inline] pub unsafe fn IsList(list: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsList.ptr)(list) }
     #[inline] pub unsafe fn IsProgram(program: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgram.ptr)(program) }
     #[inline] pub unsafe fn IsQuery(id: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsQuery.ptr)(id) }
     #[inline] pub unsafe fn IsShader(shader: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsShader.ptr)(shader) }
     #[inline] pub unsafe fn IsTexture(texture: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTexture.ptr)(texture) }
     #[inline] pub unsafe fn LightModelf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::LightModelf.ptr)(pname, param) }
     #[inline] pub unsafe fn LightModelfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::LightModelfv.ptr)(pname, params) }
     #[inline] pub unsafe fn LightModeli(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::LightModeli.ptr)(pname, param) }
     #[inline] pub unsafe fn LightModeliv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::LightModeliv.ptr)(pname, params) }
     #[inline] pub unsafe fn Lightf(light: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::Lightf.ptr)(light, pname, param) }
     #[inline] pub unsafe fn Lightfv(light: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::Lightfv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn Lighti(light: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::Lighti.ptr)(light, pname, param) }
     #[inline] pub unsafe fn Lightiv(light: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::Lightiv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn LineStipple(factor: GLint, pattern: GLushort) -> () { mem::transmute::<_, extern "system" fn(GLint, GLushort) -> ()>(storage::LineStipple.ptr)(factor, pattern) }
     #[inline] pub unsafe fn LineWidth(width: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::LineWidth.ptr)(width) }
     #[inline] pub unsafe fn LinkProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LinkProgram.ptr)(program) }
     #[inline] pub unsafe fn ListBase(base: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ListBase.ptr)(base) }
     #[inline] pub unsafe fn LoadIdentity() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::LoadIdentity.ptr)() }
     #[inline] pub unsafe fn LoadMatrixd(m: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::LoadMatrixd.ptr)(m) }
     #[inline] pub unsafe fn LoadMatrixf(m: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::LoadMatrixf.ptr)(m) }
     #[inline] pub unsafe fn LoadName(name: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LoadName.ptr)(name) }
     #[inline] pub unsafe fn LoadTransposeMatrixd(m: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::LoadTransposeMatrixd.ptr)(m) }
     #[inline] pub unsafe fn LoadTransposeMatrixf(m: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::LoadTransposeMatrixf.ptr)(m) }
     #[inline] pub unsafe fn LogicOp(opcode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::LogicOp.ptr)(opcode) }
     #[inline] pub unsafe fn Map1d(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble, GLdouble, GLint, GLint, *const GLdouble) -> ()>(storage::Map1d.ptr)(target, u1, u2, stride, order, points) }
     #[inline] pub unsafe fn Map1f(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat, GLfloat, GLint, GLint, *const GLfloat) -> ()>(storage::Map1f.ptr)(target, u1, u2, stride, order, points) }
     #[inline] pub unsafe fn Map2d(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble, GLdouble, GLint, GLint, GLdouble, GLdouble, GLint, GLint, *const GLdouble) -> ()>(storage::Map2d.ptr)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
     #[inline] pub unsafe fn Map2f(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat, GLfloat, GLint, GLint, GLfloat, GLfloat, GLint, GLint, *const GLfloat) -> ()>(storage::Map2f.ptr)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
     #[inline] pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut std::os::raw::c_void { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> *mut std::os::raw::c_void>(storage::MapBuffer.ptr)(target, access) }
     #[inline] pub unsafe fn MapGrid1d(un: GLint, u1: GLdouble, u2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble) -> ()>(storage::MapGrid1d.ptr)(un, u1, u2) }
     #[inline] pub unsafe fn MapGrid1f(un: GLint, u1: GLfloat, u2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(storage::MapGrid1f.ptr)(un, u1, u2) }
     #[inline] pub unsafe fn MapGrid2d(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLint, GLdouble, GLdouble) -> ()>(storage::MapGrid2d.ptr)(un, u1, u2, vn, v1, v2) }
     #[inline] pub unsafe fn MapGrid2f(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLint, GLfloat, GLfloat) -> ()>(storage::MapGrid2f.ptr)(un, u1, u2, vn, v1, v2) }
     #[inline] pub unsafe fn Materialf(face: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::Materialf.ptr)(face, pname, param) }
     #[inline] pub unsafe fn Materialfv(face: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::Materialfv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn Materiali(face: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::Materiali.ptr)(face, pname, param) }
     #[inline] pub unsafe fn Materialiv(face: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::Materialiv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn MatrixMode(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::MatrixMode.ptr)(mode) }
     #[inline] pub unsafe fn MultMatrixd(m: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::MultMatrixd.ptr)(m) }
     #[inline] pub unsafe fn MultMatrixf(m: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::MultMatrixf.ptr)(m) }
     #[inline] pub unsafe fn MultTransposeMatrixd(m: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::MultTransposeMatrixd.ptr)(m) }
     #[inline] pub unsafe fn MultTransposeMatrixf(m: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::MultTransposeMatrixf.ptr)(m) }
     #[inline] pub unsafe fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei) -> ()>(storage::MultiDrawArrays.ptr)(mode, first, count, drawcount) }
     #[inline] pub unsafe fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const std::os::raw::c_void, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const std::os::raw::c_void, GLsizei) -> ()>(storage::MultiDrawElements.ptr)(mode, count, type_, indices, drawcount) }
     #[inline] pub unsafe fn MultiTexCoord1d(target: GLenum, s: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble) -> ()>(storage::MultiTexCoord1d.ptr)(target, s) }
     #[inline] pub unsafe fn MultiTexCoord1dv(target: GLenum, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLdouble) -> ()>(storage::MultiTexCoord1dv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord1f(target: GLenum, s: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::MultiTexCoord1f.ptr)(target, s) }
     #[inline] pub unsafe fn MultiTexCoord1fv(target: GLenum, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::MultiTexCoord1fv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord1i(target: GLenum, s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::MultiTexCoord1i.ptr)(target, s) }
     #[inline] pub unsafe fn MultiTexCoord1iv(target: GLenum, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::MultiTexCoord1iv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord1s(target: GLenum, s: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLshort) -> ()>(storage::MultiTexCoord1s.ptr)(target, s) }
     #[inline] pub unsafe fn MultiTexCoord1sv(target: GLenum, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLshort) -> ()>(storage::MultiTexCoord1sv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord2d(target: GLenum, s: GLdouble, t: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble, GLdouble) -> ()>(storage::MultiTexCoord2d.ptr)(target, s, t) }
     #[inline] pub unsafe fn MultiTexCoord2dv(target: GLenum, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLdouble) -> ()>(storage::MultiTexCoord2dv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord2f(target: GLenum, s: GLfloat, t: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat, GLfloat) -> ()>(storage::MultiTexCoord2f.ptr)(target, s, t) }
     #[inline] pub unsafe fn MultiTexCoord2fv(target: GLenum, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::MultiTexCoord2fv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord2i(target: GLenum, s: GLint, t: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint) -> ()>(storage::MultiTexCoord2i.ptr)(target, s, t) }
     #[inline] pub unsafe fn MultiTexCoord2iv(target: GLenum, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::MultiTexCoord2iv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord2s(target: GLenum, s: GLshort, t: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLshort, GLshort) -> ()>(storage::MultiTexCoord2s.ptr)(target, s, t) }
     #[inline] pub unsafe fn MultiTexCoord2sv(target: GLenum, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLshort) -> ()>(storage::MultiTexCoord2sv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord3d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble, GLdouble, GLdouble) -> ()>(storage::MultiTexCoord3d.ptr)(target, s, t, r) }
     #[inline] pub unsafe fn MultiTexCoord3dv(target: GLenum, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLdouble) -> ()>(storage::MultiTexCoord3dv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord3f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat, GLfloat, GLfloat) -> ()>(storage::MultiTexCoord3f.ptr)(target, s, t, r) }
     #[inline] pub unsafe fn MultiTexCoord3fv(target: GLenum, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::MultiTexCoord3fv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord3i(target: GLenum, s: GLint, t: GLint, r: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint) -> ()>(storage::MultiTexCoord3i.ptr)(target, s, t, r) }
     #[inline] pub unsafe fn MultiTexCoord3iv(target: GLenum, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::MultiTexCoord3iv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord3s(target: GLenum, s: GLshort, t: GLshort, r: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLshort, GLshort, GLshort) -> ()>(storage::MultiTexCoord3s.ptr)(target, s, t, r) }
     #[inline] pub unsafe fn MultiTexCoord3sv(target: GLenum, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLshort) -> ()>(storage::MultiTexCoord3sv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord4d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::MultiTexCoord4d.ptr)(target, s, t, r, q) }
     #[inline] pub unsafe fn MultiTexCoord4dv(target: GLenum, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLdouble) -> ()>(storage::MultiTexCoord4dv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord4f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::MultiTexCoord4f.ptr)(target, s, t, r, q) }
     #[inline] pub unsafe fn MultiTexCoord4fv(target: GLenum, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::MultiTexCoord4fv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord4i(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint) -> ()>(storage::MultiTexCoord4i.ptr)(target, s, t, r, q) }
     #[inline] pub unsafe fn MultiTexCoord4iv(target: GLenum, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::MultiTexCoord4iv.ptr)(target, v) }
     #[inline] pub unsafe fn MultiTexCoord4s(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLshort, GLshort, GLshort, GLshort) -> ()>(storage::MultiTexCoord4s.ptr)(target, s, t, r, q) }
     #[inline] pub unsafe fn MultiTexCoord4sv(target: GLenum, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLshort) -> ()>(storage::MultiTexCoord4sv.ptr)(target, v) }
     #[inline] pub unsafe fn NewList(list: GLuint, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::NewList.ptr)(list, mode) }
     #[inline] pub unsafe fn Normal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLbyte, GLbyte, GLbyte) -> ()>(storage::Normal3b.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3bv(v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(*const GLbyte) -> ()>(storage::Normal3bv.ptr)(v) }
     #[inline] pub unsafe fn Normal3d(nx: GLdouble, ny: GLdouble, nz: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Normal3d.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Normal3dv.ptr)(v) }
     #[inline] pub unsafe fn Normal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Normal3f.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Normal3fv.ptr)(v) }
     #[inline] pub unsafe fn Normal3i(nx: GLint, ny: GLint, nz: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Normal3i.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Normal3iv.ptr)(v) }
     #[inline] pub unsafe fn Normal3s(nx: GLshort, ny: GLshort, nz: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::Normal3s.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Normal3sv.ptr)(v) }
     #[inline] pub unsafe fn NormalPointer(type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::NormalPointer.ptr)(type_, stride, pointer) }
     #[inline] pub unsafe fn Ortho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Ortho.ptr)(left, right, bottom, top, zNear, zFar) }
     #[inline] pub unsafe fn PassThrough(token: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::PassThrough.ptr)(token) }
     #[inline] pub unsafe fn PixelMapfv(map: GLenum, mapsize: GLsizei, values: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLfloat) -> ()>(storage::PixelMapfv.ptr)(map, mapsize, values) }
     #[inline] pub unsafe fn PixelMapuiv(map: GLenum, mapsize: GLsizei, values: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLuint) -> ()>(storage::PixelMapuiv.ptr)(map, mapsize, values) }
     #[inline] pub unsafe fn PixelMapusv(map: GLenum, mapsize: GLsizei, values: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLushort) -> ()>(storage::PixelMapusv.ptr)(map, mapsize, values) }
     #[inline] pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PixelStoref.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelStorei.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelTransferf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PixelTransferf.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelTransferi(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelTransferi.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelZoom(xfactor: GLfloat, yfactor: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PixelZoom.ptr)(xfactor, yfactor) }
     #[inline] pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PointParameterf.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::PointParameterfv.ptr)(pname, params) }
     #[inline] pub unsafe fn PointParameteri(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PointParameteri.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::PointParameteriv.ptr)(pname, params) }
     #[inline] pub unsafe fn PointSize(size: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::PointSize.ptr)(size) }
     #[inline] pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::PolygonMode.ptr)(face, mode) }
     #[inline] pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PolygonOffset.ptr)(factor, units) }
     #[inline] pub unsafe fn PolygonStipple(mask: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::PolygonStipple.ptr)(mask) }
     #[inline] pub unsafe fn PopAttrib() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopAttrib.ptr)() }
     #[inline] pub unsafe fn PopClientAttrib() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopClientAttrib.ptr)() }
     #[inline] pub unsafe fn PopMatrix() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopMatrix.ptr)() }
     #[inline] pub unsafe fn PopName() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopName.ptr)() }
     #[inline] pub unsafe fn PrioritizeTextures(n: GLsizei, textures: *const GLuint, priorities: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint, *const GLfloat) -> ()>(storage::PrioritizeTextures.ptr)(n, textures, priorities) }
     #[inline] pub unsafe fn PushAttrib(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::PushAttrib.ptr)(mask) }
     #[inline] pub unsafe fn PushClientAttrib(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::PushClientAttrib.ptr)(mask) }
     #[inline] pub unsafe fn PushMatrix() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PushMatrix.ptr)() }
     #[inline] pub unsafe fn PushName(name: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::PushName.ptr)(name) }
     #[inline] pub unsafe fn RasterPos2d(x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::RasterPos2d.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::RasterPos2dv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos2f(x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::RasterPos2f.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::RasterPos2fv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos2i(x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::RasterPos2i.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::RasterPos2iv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos2s(x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort) -> ()>(storage::RasterPos2s.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::RasterPos2sv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3d(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::RasterPos3d.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::RasterPos3dv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::RasterPos3f.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::RasterPos3fv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3i(x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::RasterPos3i.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::RasterPos3iv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3s(x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::RasterPos3s.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::RasterPos3sv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::RasterPos4d.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::RasterPos4dv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::RasterPos4f.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::RasterPos4fv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::RasterPos4i.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::RasterPos4iv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::RasterPos4s.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::RasterPos4sv.ptr)(v) }
     #[inline] pub unsafe fn ReadBuffer(src: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ReadBuffer.ptr)(src) }
     #[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::ReadPixels.ptr)(x, y, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn Rectd(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Rectd.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectdv(v1: *const GLdouble, v2: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble, *const GLdouble) -> ()>(storage::Rectdv.ptr)(v1, v2) }
     #[inline] pub unsafe fn Rectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Rectf.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectfv(v1: *const GLfloat, v2: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat, *const GLfloat) -> ()>(storage::Rectfv.ptr)(v1, v2) }
     #[inline] pub unsafe fn Recti(x1: GLint, y1: GLint, x2: GLint, y2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Recti.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectiv(v1: *const GLint, v2: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint, *const GLint) -> ()>(storage::Rectiv.ptr)(v1, v2) }
     #[inline] pub unsafe fn Rects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::Rects.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectsv(v1: *const GLshort, v2: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort, *const GLshort) -> ()>(storage::Rectsv.ptr)(v1, v2) }
     #[inline] pub unsafe fn RenderMode(mode: GLenum) -> GLint { mem::transmute::<_, extern "system" fn(GLenum) -> GLint>(storage::RenderMode.ptr)(mode) }
     #[inline] pub unsafe fn Rotated(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Rotated.ptr)(angle, x, y, z) }
     #[inline] pub unsafe fn Rotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Rotatef.ptr)(angle, x, y, z) }
     #[inline] pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>(storage::SampleCoverage.ptr)(value, invert) }
     #[inline] pub unsafe fn Scaled(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Scaled.ptr)(x, y, z) }
     #[inline] pub unsafe fn Scalef(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Scalef.ptr)(x, y, z) }
     #[inline] pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Scissor.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn SecondaryColor3b(red: GLbyte, green: GLbyte, blue: GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLbyte, GLbyte, GLbyte) -> ()>(storage::SecondaryColor3b.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3bv(v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(*const GLbyte) -> ()>(storage::SecondaryColor3bv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColor3d(red: GLdouble, green: GLdouble, blue: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::SecondaryColor3d.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::SecondaryColor3dv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColor3f(red: GLfloat, green: GLfloat, blue: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::SecondaryColor3f.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::SecondaryColor3fv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColor3i(red: GLint, green: GLint, blue: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::SecondaryColor3i.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::SecondaryColor3iv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColor3s(red: GLshort, green: GLshort, blue: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::SecondaryColor3s.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::SecondaryColor3sv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLubyte, GLubyte, GLubyte) -> ()>(storage::SecondaryColor3ub.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3ubv(v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::SecondaryColor3ubv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColor3ui(red: GLuint, green: GLuint, blue: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::SecondaryColor3ui.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3uiv(v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(*const GLuint) -> ()>(storage::SecondaryColor3uiv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColor3us(red: GLushort, green: GLushort, blue: GLushort) -> () { mem::transmute::<_, extern "system" fn(GLushort, GLushort, GLushort) -> ()>(storage::SecondaryColor3us.ptr)(red, green, blue) }
     #[inline] pub unsafe fn SecondaryColor3usv(v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(*const GLushort) -> ()>(storage::SecondaryColor3usv.ptr)(v) }
     #[inline] pub unsafe fn SecondaryColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::SecondaryColorPointer.ptr)(size, type_, stride, pointer) }
     #[inline] pub unsafe fn SelectBuffer(size: GLsizei, buffer: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::SelectBuffer.ptr)(size, buffer) }
     #[inline] pub unsafe fn ShadeModel(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ShadeModel.ptr)(mode) }
     #[inline] pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>(storage::ShaderSource.ptr)(shader, count, string, length) }
     #[inline] pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(storage::StencilFunc.ptr)(func, ref_, mask) }
     #[inline] pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>(storage::StencilFuncSeparate.ptr)(face, func, ref_, mask) }
     #[inline] pub unsafe fn StencilMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::StencilMask.ptr)(mask) }
     #[inline] pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::StencilMaskSeparate.ptr)(face, mask) }
     #[inline] pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(storage::StencilOp.ptr)(fail, zfail, zpass) }
     #[inline] pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::StencilOpSeparate.ptr)(face, sfail, dpfail, dppass) }
     #[inline] pub unsafe fn TexCoord1d(s: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::TexCoord1d.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord1dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord1f(s: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::TexCoord1f.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord1fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord1i(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::TexCoord1i.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord1iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord1s(s: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort) -> ()>(storage::TexCoord1s.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord1sv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2d(s: GLdouble, t: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::TexCoord2d.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord2dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2f(s: GLfloat, t: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::TexCoord2f.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord2fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2i(s: GLint, t: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::TexCoord2i.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord2iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2s(s: GLshort, t: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort) -> ()>(storage::TexCoord2s.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord2sv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3d(s: GLdouble, t: GLdouble, r: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::TexCoord3d.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord3dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3f(s: GLfloat, t: GLfloat, r: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::TexCoord3f.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord3fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3i(s: GLint, t: GLint, r: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::TexCoord3i.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord3iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3s(s: GLshort, t: GLshort, r: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::TexCoord3s.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord3sv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4d(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::TexCoord4d.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord4dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4f(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::TexCoord4f.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord4fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::TexCoord4i.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord4iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::TexCoord4s.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord4sv.ptr)(v) }
     #[inline] pub unsafe fn TexCoordPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::TexCoordPointer.ptr)(size, type_, stride, pointer) }
     #[inline] pub unsafe fn TexEnvf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexEnvf.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexEnvfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexEnvfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexEnvi(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexEnvi.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexEnviv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexEnviv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexGend(coord: GLenum, pname: GLenum, param: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLdouble) -> ()>(storage::TexGend.ptr)(coord, pname, param) }
     #[inline] pub unsafe fn TexGendv(coord: GLenum, pname: GLenum, params: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLdouble) -> ()>(storage::TexGendv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn TexGenf(coord: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexGenf.ptr)(coord, pname, param) }
     #[inline] pub unsafe fn TexGenfv(coord: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexGenfv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn TexGeni(coord: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexGeni.ptr)(coord, pname, param) }
     #[inline] pub unsafe fn TexGeniv(coord: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexGeniv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage1D.ptr)(target, level, internalformat, width, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage2D.ptr)(target, level, internalformat, width, height, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage3D.ptr)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexParameterf.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexParameteri.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage1D.ptr)(target, level, xoffset, width, format, type_, pixels) }
     #[inline] pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
     #[inline] pub unsafe fn Translated(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Translated.ptr)(x, y, z) }
     #[inline] pub unsafe fn Translatef(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Translatef.ptr)(x, y, z) }
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
     #[inline] pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::UnmapBuffer.ptr)(target) }
     #[inline] pub unsafe fn UseProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::UseProgram.ptr)(program) }
     #[inline] pub unsafe fn ValidateProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgram.ptr)(program) }
     #[inline] pub unsafe fn Vertex2d(x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::Vertex2d.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Vertex2dv.ptr)(v) }
     #[inline] pub unsafe fn Vertex2f(x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::Vertex2f.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Vertex2fv.ptr)(v) }
     #[inline] pub unsafe fn Vertex2i(x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::Vertex2i.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Vertex2iv.ptr)(v) }
     #[inline] pub unsafe fn Vertex2s(x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort) -> ()>(storage::Vertex2s.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Vertex2sv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3d(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Vertex3d.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Vertex3dv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3f(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Vertex3f.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Vertex3fv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3i(x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Vertex3i.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Vertex3iv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3s(x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::Vertex3s.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Vertex3sv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Vertex4d.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Vertex4dv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Vertex4f.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Vertex4fv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4i(x: GLint, y: GLint, z: GLint, w: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Vertex4i.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Vertex4iv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::Vertex4s.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Vertex4sv.ptr)(v) }
     #[inline] pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(storage::VertexAttrib1d.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib1dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>(storage::VertexAttrib1f.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib1fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort) -> ()>(storage::VertexAttrib1s.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib1sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::VertexAttrib2d.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib2dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>(storage::VertexAttrib2f.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib2fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort) -> ()>(storage::VertexAttrib2s.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib2sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib3d.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib3dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib3f.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib3fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib3s.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib3sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4Nbv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4Niv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4Nsv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte) -> ()>(storage::VertexAttrib4Nub.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4Nubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4Nuiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4Nusv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4bv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib4d.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib4dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib4f.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib4fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib4s.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4ubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4usv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexAttribPointer.ptr)(index, size, type_, normalized, stride, pointer) }
     #[inline] pub unsafe fn VertexPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexPointer.ptr)(size, type_, stride, pointer) }
     #[inline] pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Viewport.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn WindowPos2d(x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::WindowPos2d.ptr)(x, y) }
     #[inline] pub unsafe fn WindowPos2dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::WindowPos2dv.ptr)(v) }
     #[inline] pub unsafe fn WindowPos2f(x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::WindowPos2f.ptr)(x, y) }
     #[inline] pub unsafe fn WindowPos2fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::WindowPos2fv.ptr)(v) }
     #[inline] pub unsafe fn WindowPos2i(x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::WindowPos2i.ptr)(x, y) }
     #[inline] pub unsafe fn WindowPos2iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::WindowPos2iv.ptr)(v) }
     #[inline] pub unsafe fn WindowPos2s(x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort) -> ()>(storage::WindowPos2s.ptr)(x, y) }
     #[inline] pub unsafe fn WindowPos2sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::WindowPos2sv.ptr)(v) }
     #[inline] pub unsafe fn WindowPos3d(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::WindowPos3d.ptr)(x, y, z) }
     #[inline] pub unsafe fn WindowPos3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::WindowPos3dv.ptr)(v) }
     #[inline] pub unsafe fn WindowPos3f(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::WindowPos3f.ptr)(x, y, z) }
     #[inline] pub unsafe fn WindowPos3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::WindowPos3fv.ptr)(v) }
     #[inline] pub unsafe fn WindowPos3i(x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::WindowPos3i.ptr)(x, y, z) }
     #[inline] pub unsafe fn WindowPos3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::WindowPos3iv.ptr)(v) }
     #[inline] pub unsafe fn WindowPos3s(x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::WindowPos3s.ptr)(x, y, z) }
     #[inline] pub unsafe fn WindowPos3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::WindowPos3sv.ptr)(v) }
}

mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw;

     pub static mut Accum: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ActiveTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut AlphaFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut AreTexturesResident: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ArrayElement: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut AttachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Begin: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Bitmap: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquationSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CallList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CallLists: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Clear: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearAccum: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearDepth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearStencil: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClientActiveTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClipPlane: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3b: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3ub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3us: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4b: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4ub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4us: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMaterial: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompileShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CullFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteLists: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteQueries: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DetachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Disable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DisableClientState: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DisableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawRangeElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EdgeFlag: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EdgeFlagPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EdgeFlagv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Enable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EnableClientState: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EnableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut End: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalMesh1: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalMesh2: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalPoint1: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalPoint2: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FeedbackBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Finish: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Flush: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FogCoordPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FogCoordd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FogCoorddv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FogCoordf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FogCoordfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FrontFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Frustum: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenLists: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenQueries: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniform: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttachedShaders: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBooleanv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetClipPlane: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetCompressedTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetDoublev: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetError: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFloatv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetIntegerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetLightfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetLightiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMapdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMapfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMapiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMaterialfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMaterialiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPixelMapfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPixelMapuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPixelMapusv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPolygonStipple: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjectiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjectuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetString: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexEnvfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexEnviv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexGendv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexGenfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexGeniv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Hint: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IndexMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IndexPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexs: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexsv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InitNames: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InterleavedArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsEnabled: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModelf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModelfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModeli: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModeliv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lightf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lightfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lighti: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lightiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LineStipple: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LineWidth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LinkProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ListBase: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadIdentity: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadMatrixd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadMatrixf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadTransposeMatrixd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadTransposeMatrixf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LogicOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materialf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materialfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materiali: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materialiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MatrixMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultMatrixd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultMatrixf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultTransposeMatrixd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultTransposeMatrixf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord1sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiTexCoord4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NewList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3b: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NormalPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Ortho: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PassThrough: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelMapfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelMapuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelMapusv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStoref: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStorei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelTransferf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelTransferi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelZoom: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointSize: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonOffset: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonStipple: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopClientAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopMatrix: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PrioritizeTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushClientAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushMatrix: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Recti: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rects: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectsv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RenderMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rotated: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rotatef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SampleCoverage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scaled: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scalef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scissor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3b: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3ub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3us: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColor3usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SecondaryColorPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SelectBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShadeModel: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMaskSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOpSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoordPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnvf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnvfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnvi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnviv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGend: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGendv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGenf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGenfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGeni: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGeniv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Translated: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Translatef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
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
     pub static mut UnmapBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UseProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ValidateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nbv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Niv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nsv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nusv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Viewport: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WindowPos3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
}

pub fn load<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
    unsafe {
         storage::Accum.load(&mut loadfn, "glAccum");
         storage::ActiveTexture.load(&mut loadfn, "glActiveTexture");
         storage::AlphaFunc.load(&mut loadfn, "glAlphaFunc");
         storage::AreTexturesResident.load(&mut loadfn, "glAreTexturesResident");
         storage::ArrayElement.load(&mut loadfn, "glArrayElement");
         storage::AttachShader.load(&mut loadfn, "glAttachShader");
         storage::Begin.load(&mut loadfn, "glBegin");
         storage::BeginQuery.load(&mut loadfn, "glBeginQuery");
         storage::BindAttribLocation.load(&mut loadfn, "glBindAttribLocation");
         storage::BindBuffer.load(&mut loadfn, "glBindBuffer");
         storage::BindTexture.load(&mut loadfn, "glBindTexture");
         storage::Bitmap.load(&mut loadfn, "glBitmap");
         storage::BlendColor.load(&mut loadfn, "glBlendColor");
         storage::BlendEquation.load(&mut loadfn, "glBlendEquation");
         storage::BlendEquationSeparate.load(&mut loadfn, "glBlendEquationSeparate");
         storage::BlendFunc.load(&mut loadfn, "glBlendFunc");
         storage::BlendFuncSeparate.load(&mut loadfn, "glBlendFuncSeparate");
         storage::BufferData.load(&mut loadfn, "glBufferData");
         storage::BufferSubData.load(&mut loadfn, "glBufferSubData");
         storage::CallList.load(&mut loadfn, "glCallList");
         storage::CallLists.load(&mut loadfn, "glCallLists");
         storage::Clear.load(&mut loadfn, "glClear");
         storage::ClearAccum.load(&mut loadfn, "glClearAccum");
         storage::ClearColor.load(&mut loadfn, "glClearColor");
         storage::ClearDepth.load(&mut loadfn, "glClearDepth");
         storage::ClearIndex.load(&mut loadfn, "glClearIndex");
         storage::ClearStencil.load(&mut loadfn, "glClearStencil");
         storage::ClientActiveTexture.load(&mut loadfn, "glClientActiveTexture");
         storage::ClipPlane.load(&mut loadfn, "glClipPlane");
         storage::Color3b.load(&mut loadfn, "glColor3b");
         storage::Color3bv.load(&mut loadfn, "glColor3bv");
         storage::Color3d.load(&mut loadfn, "glColor3d");
         storage::Color3dv.load(&mut loadfn, "glColor3dv");
         storage::Color3f.load(&mut loadfn, "glColor3f");
         storage::Color3fv.load(&mut loadfn, "glColor3fv");
         storage::Color3i.load(&mut loadfn, "glColor3i");
         storage::Color3iv.load(&mut loadfn, "glColor3iv");
         storage::Color3s.load(&mut loadfn, "glColor3s");
         storage::Color3sv.load(&mut loadfn, "glColor3sv");
         storage::Color3ub.load(&mut loadfn, "glColor3ub");
         storage::Color3ubv.load(&mut loadfn, "glColor3ubv");
         storage::Color3ui.load(&mut loadfn, "glColor3ui");
         storage::Color3uiv.load(&mut loadfn, "glColor3uiv");
         storage::Color3us.load(&mut loadfn, "glColor3us");
         storage::Color3usv.load(&mut loadfn, "glColor3usv");
         storage::Color4b.load(&mut loadfn, "glColor4b");
         storage::Color4bv.load(&mut loadfn, "glColor4bv");
         storage::Color4d.load(&mut loadfn, "glColor4d");
         storage::Color4dv.load(&mut loadfn, "glColor4dv");
         storage::Color4f.load(&mut loadfn, "glColor4f");
         storage::Color4fv.load(&mut loadfn, "glColor4fv");
         storage::Color4i.load(&mut loadfn, "glColor4i");
         storage::Color4iv.load(&mut loadfn, "glColor4iv");
         storage::Color4s.load(&mut loadfn, "glColor4s");
         storage::Color4sv.load(&mut loadfn, "glColor4sv");
         storage::Color4ub.load(&mut loadfn, "glColor4ub");
         storage::Color4ubv.load(&mut loadfn, "glColor4ubv");
         storage::Color4ui.load(&mut loadfn, "glColor4ui");
         storage::Color4uiv.load(&mut loadfn, "glColor4uiv");
         storage::Color4us.load(&mut loadfn, "glColor4us");
         storage::Color4usv.load(&mut loadfn, "glColor4usv");
         storage::ColorMask.load(&mut loadfn, "glColorMask");
         storage::ColorMaterial.load(&mut loadfn, "glColorMaterial");
         storage::ColorPointer.load(&mut loadfn, "glColorPointer");
         storage::CompileShader.load(&mut loadfn, "glCompileShader");
         storage::CompressedTexImage1D.load(&mut loadfn, "glCompressedTexImage1D");
         storage::CompressedTexImage2D.load(&mut loadfn, "glCompressedTexImage2D");
         storage::CompressedTexImage3D.load(&mut loadfn, "glCompressedTexImage3D");
         storage::CompressedTexSubImage1D.load(&mut loadfn, "glCompressedTexSubImage1D");
         storage::CompressedTexSubImage2D.load(&mut loadfn, "glCompressedTexSubImage2D");
         storage::CompressedTexSubImage3D.load(&mut loadfn, "glCompressedTexSubImage3D");
         storage::CopyPixels.load(&mut loadfn, "glCopyPixels");
         storage::CopyTexImage1D.load(&mut loadfn, "glCopyTexImage1D");
         storage::CopyTexImage2D.load(&mut loadfn, "glCopyTexImage2D");
         storage::CopyTexSubImage1D.load(&mut loadfn, "glCopyTexSubImage1D");
         storage::CopyTexSubImage2D.load(&mut loadfn, "glCopyTexSubImage2D");
         storage::CopyTexSubImage3D.load(&mut loadfn, "glCopyTexSubImage3D");
         storage::CreateProgram.load(&mut loadfn, "glCreateProgram");
         storage::CreateShader.load(&mut loadfn, "glCreateShader");
         storage::CullFace.load(&mut loadfn, "glCullFace");
         storage::DeleteBuffers.load(&mut loadfn, "glDeleteBuffers");
         storage::DeleteLists.load(&mut loadfn, "glDeleteLists");
         storage::DeleteProgram.load(&mut loadfn, "glDeleteProgram");
         storage::DeleteQueries.load(&mut loadfn, "glDeleteQueries");
         storage::DeleteShader.load(&mut loadfn, "glDeleteShader");
         storage::DeleteTextures.load(&mut loadfn, "glDeleteTextures");
         storage::DepthFunc.load(&mut loadfn, "glDepthFunc");
         storage::DepthMask.load(&mut loadfn, "glDepthMask");
         storage::DepthRange.load(&mut loadfn, "glDepthRange");
         storage::DetachShader.load(&mut loadfn, "glDetachShader");
         storage::Disable.load(&mut loadfn, "glDisable");
         storage::DisableClientState.load(&mut loadfn, "glDisableClientState");
         storage::DisableVertexAttribArray.load(&mut loadfn, "glDisableVertexAttribArray");
         storage::DrawArrays.load(&mut loadfn, "glDrawArrays");
         storage::DrawBuffer.load(&mut loadfn, "glDrawBuffer");
         storage::DrawBuffers.load(&mut loadfn, "glDrawBuffers");
         storage::DrawElements.load(&mut loadfn, "glDrawElements");
         storage::DrawPixels.load(&mut loadfn, "glDrawPixels");
         storage::DrawRangeElements.load(&mut loadfn, "glDrawRangeElements");
         storage::EdgeFlag.load(&mut loadfn, "glEdgeFlag");
         storage::EdgeFlagPointer.load(&mut loadfn, "glEdgeFlagPointer");
         storage::EdgeFlagv.load(&mut loadfn, "glEdgeFlagv");
         storage::Enable.load(&mut loadfn, "glEnable");
         storage::EnableClientState.load(&mut loadfn, "glEnableClientState");
         storage::EnableVertexAttribArray.load(&mut loadfn, "glEnableVertexAttribArray");
         storage::End.load(&mut loadfn, "glEnd");
         storage::EndList.load(&mut loadfn, "glEndList");
         storage::EndQuery.load(&mut loadfn, "glEndQuery");
         storage::EvalCoord1d.load(&mut loadfn, "glEvalCoord1d");
         storage::EvalCoord1dv.load(&mut loadfn, "glEvalCoord1dv");
         storage::EvalCoord1f.load(&mut loadfn, "glEvalCoord1f");
         storage::EvalCoord1fv.load(&mut loadfn, "glEvalCoord1fv");
         storage::EvalCoord2d.load(&mut loadfn, "glEvalCoord2d");
         storage::EvalCoord2dv.load(&mut loadfn, "glEvalCoord2dv");
         storage::EvalCoord2f.load(&mut loadfn, "glEvalCoord2f");
         storage::EvalCoord2fv.load(&mut loadfn, "glEvalCoord2fv");
         storage::EvalMesh1.load(&mut loadfn, "glEvalMesh1");
         storage::EvalMesh2.load(&mut loadfn, "glEvalMesh2");
         storage::EvalPoint1.load(&mut loadfn, "glEvalPoint1");
         storage::EvalPoint2.load(&mut loadfn, "glEvalPoint2");
         storage::FeedbackBuffer.load(&mut loadfn, "glFeedbackBuffer");
         storage::Finish.load(&mut loadfn, "glFinish");
         storage::Flush.load(&mut loadfn, "glFlush");
         storage::FogCoordPointer.load(&mut loadfn, "glFogCoordPointer");
         storage::FogCoordd.load(&mut loadfn, "glFogCoordd");
         storage::FogCoorddv.load(&mut loadfn, "glFogCoorddv");
         storage::FogCoordf.load(&mut loadfn, "glFogCoordf");
         storage::FogCoordfv.load(&mut loadfn, "glFogCoordfv");
         storage::Fogf.load(&mut loadfn, "glFogf");
         storage::Fogfv.load(&mut loadfn, "glFogfv");
         storage::Fogi.load(&mut loadfn, "glFogi");
         storage::Fogiv.load(&mut loadfn, "glFogiv");
         storage::FrontFace.load(&mut loadfn, "glFrontFace");
         storage::Frustum.load(&mut loadfn, "glFrustum");
         storage::GenBuffers.load(&mut loadfn, "glGenBuffers");
         storage::GenLists.load(&mut loadfn, "glGenLists");
         storage::GenQueries.load(&mut loadfn, "glGenQueries");
         storage::GenTextures.load(&mut loadfn, "glGenTextures");
         storage::GetActiveAttrib.load(&mut loadfn, "glGetActiveAttrib");
         storage::GetActiveUniform.load(&mut loadfn, "glGetActiveUniform");
         storage::GetAttachedShaders.load(&mut loadfn, "glGetAttachedShaders");
         storage::GetAttribLocation.load(&mut loadfn, "glGetAttribLocation");
         storage::GetBooleanv.load(&mut loadfn, "glGetBooleanv");
         storage::GetBufferParameteriv.load(&mut loadfn, "glGetBufferParameteriv");
         storage::GetBufferPointerv.load(&mut loadfn, "glGetBufferPointerv");
         storage::GetBufferSubData.load(&mut loadfn, "glGetBufferSubData");
         storage::GetClipPlane.load(&mut loadfn, "glGetClipPlane");
         storage::GetCompressedTexImage.load(&mut loadfn, "glGetCompressedTexImage");
         storage::GetDoublev.load(&mut loadfn, "glGetDoublev");
         storage::GetError.load(&mut loadfn, "glGetError");
         storage::GetFloatv.load(&mut loadfn, "glGetFloatv");
         storage::GetIntegerv.load(&mut loadfn, "glGetIntegerv");
         storage::GetLightfv.load(&mut loadfn, "glGetLightfv");
         storage::GetLightiv.load(&mut loadfn, "glGetLightiv");
         storage::GetMapdv.load(&mut loadfn, "glGetMapdv");
         storage::GetMapfv.load(&mut loadfn, "glGetMapfv");
         storage::GetMapiv.load(&mut loadfn, "glGetMapiv");
         storage::GetMaterialfv.load(&mut loadfn, "glGetMaterialfv");
         storage::GetMaterialiv.load(&mut loadfn, "glGetMaterialiv");
         storage::GetPixelMapfv.load(&mut loadfn, "glGetPixelMapfv");
         storage::GetPixelMapuiv.load(&mut loadfn, "glGetPixelMapuiv");
         storage::GetPixelMapusv.load(&mut loadfn, "glGetPixelMapusv");
         storage::GetPointerv.load(&mut loadfn, "glGetPointerv");
         storage::GetPolygonStipple.load(&mut loadfn, "glGetPolygonStipple");
         storage::GetProgramInfoLog.load(&mut loadfn, "glGetProgramInfoLog");
         storage::GetProgramiv.load(&mut loadfn, "glGetProgramiv");
         storage::GetQueryObjectiv.load(&mut loadfn, "glGetQueryObjectiv");
         storage::GetQueryObjectuiv.load(&mut loadfn, "glGetQueryObjectuiv");
         storage::GetQueryiv.load(&mut loadfn, "glGetQueryiv");
         storage::GetShaderInfoLog.load(&mut loadfn, "glGetShaderInfoLog");
         storage::GetShaderSource.load(&mut loadfn, "glGetShaderSource");
         storage::GetShaderiv.load(&mut loadfn, "glGetShaderiv");
         storage::GetString.load(&mut loadfn, "glGetString");
         storage::GetTexEnvfv.load(&mut loadfn, "glGetTexEnvfv");
         storage::GetTexEnviv.load(&mut loadfn, "glGetTexEnviv");
         storage::GetTexGendv.load(&mut loadfn, "glGetTexGendv");
         storage::GetTexGenfv.load(&mut loadfn, "glGetTexGenfv");
         storage::GetTexGeniv.load(&mut loadfn, "glGetTexGeniv");
         storage::GetTexImage.load(&mut loadfn, "glGetTexImage");
         storage::GetTexLevelParameterfv.load(&mut loadfn, "glGetTexLevelParameterfv");
         storage::GetTexLevelParameteriv.load(&mut loadfn, "glGetTexLevelParameteriv");
         storage::GetTexParameterfv.load(&mut loadfn, "glGetTexParameterfv");
         storage::GetTexParameteriv.load(&mut loadfn, "glGetTexParameteriv");
         storage::GetUniformLocation.load(&mut loadfn, "glGetUniformLocation");
         storage::GetUniformfv.load(&mut loadfn, "glGetUniformfv");
         storage::GetUniformiv.load(&mut loadfn, "glGetUniformiv");
         storage::GetVertexAttribPointerv.load(&mut loadfn, "glGetVertexAttribPointerv");
         storage::GetVertexAttribdv.load(&mut loadfn, "glGetVertexAttribdv");
         storage::GetVertexAttribfv.load(&mut loadfn, "glGetVertexAttribfv");
         storage::GetVertexAttribiv.load(&mut loadfn, "glGetVertexAttribiv");
         storage::Hint.load(&mut loadfn, "glHint");
         storage::IndexMask.load(&mut loadfn, "glIndexMask");
         storage::IndexPointer.load(&mut loadfn, "glIndexPointer");
         storage::Indexd.load(&mut loadfn, "glIndexd");
         storage::Indexdv.load(&mut loadfn, "glIndexdv");
         storage::Indexf.load(&mut loadfn, "glIndexf");
         storage::Indexfv.load(&mut loadfn, "glIndexfv");
         storage::Indexi.load(&mut loadfn, "glIndexi");
         storage::Indexiv.load(&mut loadfn, "glIndexiv");
         storage::Indexs.load(&mut loadfn, "glIndexs");
         storage::Indexsv.load(&mut loadfn, "glIndexsv");
         storage::Indexub.load(&mut loadfn, "glIndexub");
         storage::Indexubv.load(&mut loadfn, "glIndexubv");
         storage::InitNames.load(&mut loadfn, "glInitNames");
         storage::InterleavedArrays.load(&mut loadfn, "glInterleavedArrays");
         storage::IsBuffer.load(&mut loadfn, "glIsBuffer");
         storage::IsEnabled.load(&mut loadfn, "glIsEnabled");
         storage::IsList.load(&mut loadfn, "glIsList");
         storage::IsProgram.load(&mut loadfn, "glIsProgram");
         storage::IsQuery.load(&mut loadfn, "glIsQuery");
         storage::IsShader.load(&mut loadfn, "glIsShader");
         storage::IsTexture.load(&mut loadfn, "glIsTexture");
         storage::LightModelf.load(&mut loadfn, "glLightModelf");
         storage::LightModelfv.load(&mut loadfn, "glLightModelfv");
         storage::LightModeli.load(&mut loadfn, "glLightModeli");
         storage::LightModeliv.load(&mut loadfn, "glLightModeliv");
         storage::Lightf.load(&mut loadfn, "glLightf");
         storage::Lightfv.load(&mut loadfn, "glLightfv");
         storage::Lighti.load(&mut loadfn, "glLighti");
         storage::Lightiv.load(&mut loadfn, "glLightiv");
         storage::LineStipple.load(&mut loadfn, "glLineStipple");
         storage::LineWidth.load(&mut loadfn, "glLineWidth");
         storage::LinkProgram.load(&mut loadfn, "glLinkProgram");
         storage::ListBase.load(&mut loadfn, "glListBase");
         storage::LoadIdentity.load(&mut loadfn, "glLoadIdentity");
         storage::LoadMatrixd.load(&mut loadfn, "glLoadMatrixd");
         storage::LoadMatrixf.load(&mut loadfn, "glLoadMatrixf");
         storage::LoadName.load(&mut loadfn, "glLoadName");
         storage::LoadTransposeMatrixd.load(&mut loadfn, "glLoadTransposeMatrixd");
         storage::LoadTransposeMatrixf.load(&mut loadfn, "glLoadTransposeMatrixf");
         storage::LogicOp.load(&mut loadfn, "glLogicOp");
         storage::Map1d.load(&mut loadfn, "glMap1d");
         storage::Map1f.load(&mut loadfn, "glMap1f");
         storage::Map2d.load(&mut loadfn, "glMap2d");
         storage::Map2f.load(&mut loadfn, "glMap2f");
         storage::MapBuffer.load(&mut loadfn, "glMapBuffer");
         storage::MapGrid1d.load(&mut loadfn, "glMapGrid1d");
         storage::MapGrid1f.load(&mut loadfn, "glMapGrid1f");
         storage::MapGrid2d.load(&mut loadfn, "glMapGrid2d");
         storage::MapGrid2f.load(&mut loadfn, "glMapGrid2f");
         storage::Materialf.load(&mut loadfn, "glMaterialf");
         storage::Materialfv.load(&mut loadfn, "glMaterialfv");
         storage::Materiali.load(&mut loadfn, "glMateriali");
         storage::Materialiv.load(&mut loadfn, "glMaterialiv");
         storage::MatrixMode.load(&mut loadfn, "glMatrixMode");
         storage::MultMatrixd.load(&mut loadfn, "glMultMatrixd");
         storage::MultMatrixf.load(&mut loadfn, "glMultMatrixf");
         storage::MultTransposeMatrixd.load(&mut loadfn, "glMultTransposeMatrixd");
         storage::MultTransposeMatrixf.load(&mut loadfn, "glMultTransposeMatrixf");
         storage::MultiDrawArrays.load(&mut loadfn, "glMultiDrawArrays");
         storage::MultiDrawElements.load(&mut loadfn, "glMultiDrawElements");
         storage::MultiTexCoord1d.load(&mut loadfn, "glMultiTexCoord1d");
         storage::MultiTexCoord1dv.load(&mut loadfn, "glMultiTexCoord1dv");
         storage::MultiTexCoord1f.load(&mut loadfn, "glMultiTexCoord1f");
         storage::MultiTexCoord1fv.load(&mut loadfn, "glMultiTexCoord1fv");
         storage::MultiTexCoord1i.load(&mut loadfn, "glMultiTexCoord1i");
         storage::MultiTexCoord1iv.load(&mut loadfn, "glMultiTexCoord1iv");
         storage::MultiTexCoord1s.load(&mut loadfn, "glMultiTexCoord1s");
         storage::MultiTexCoord1sv.load(&mut loadfn, "glMultiTexCoord1sv");
         storage::MultiTexCoord2d.load(&mut loadfn, "glMultiTexCoord2d");
         storage::MultiTexCoord2dv.load(&mut loadfn, "glMultiTexCoord2dv");
         storage::MultiTexCoord2f.load(&mut loadfn, "glMultiTexCoord2f");
         storage::MultiTexCoord2fv.load(&mut loadfn, "glMultiTexCoord2fv");
         storage::MultiTexCoord2i.load(&mut loadfn, "glMultiTexCoord2i");
         storage::MultiTexCoord2iv.load(&mut loadfn, "glMultiTexCoord2iv");
         storage::MultiTexCoord2s.load(&mut loadfn, "glMultiTexCoord2s");
         storage::MultiTexCoord2sv.load(&mut loadfn, "glMultiTexCoord2sv");
         storage::MultiTexCoord3d.load(&mut loadfn, "glMultiTexCoord3d");
         storage::MultiTexCoord3dv.load(&mut loadfn, "glMultiTexCoord3dv");
         storage::MultiTexCoord3f.load(&mut loadfn, "glMultiTexCoord3f");
         storage::MultiTexCoord3fv.load(&mut loadfn, "glMultiTexCoord3fv");
         storage::MultiTexCoord3i.load(&mut loadfn, "glMultiTexCoord3i");
         storage::MultiTexCoord3iv.load(&mut loadfn, "glMultiTexCoord3iv");
         storage::MultiTexCoord3s.load(&mut loadfn, "glMultiTexCoord3s");
         storage::MultiTexCoord3sv.load(&mut loadfn, "glMultiTexCoord3sv");
         storage::MultiTexCoord4d.load(&mut loadfn, "glMultiTexCoord4d");
         storage::MultiTexCoord4dv.load(&mut loadfn, "glMultiTexCoord4dv");
         storage::MultiTexCoord4f.load(&mut loadfn, "glMultiTexCoord4f");
         storage::MultiTexCoord4fv.load(&mut loadfn, "glMultiTexCoord4fv");
         storage::MultiTexCoord4i.load(&mut loadfn, "glMultiTexCoord4i");
         storage::MultiTexCoord4iv.load(&mut loadfn, "glMultiTexCoord4iv");
         storage::MultiTexCoord4s.load(&mut loadfn, "glMultiTexCoord4s");
         storage::MultiTexCoord4sv.load(&mut loadfn, "glMultiTexCoord4sv");
         storage::NewList.load(&mut loadfn, "glNewList");
         storage::Normal3b.load(&mut loadfn, "glNormal3b");
         storage::Normal3bv.load(&mut loadfn, "glNormal3bv");
         storage::Normal3d.load(&mut loadfn, "glNormal3d");
         storage::Normal3dv.load(&mut loadfn, "glNormal3dv");
         storage::Normal3f.load(&mut loadfn, "glNormal3f");
         storage::Normal3fv.load(&mut loadfn, "glNormal3fv");
         storage::Normal3i.load(&mut loadfn, "glNormal3i");
         storage::Normal3iv.load(&mut loadfn, "glNormal3iv");
         storage::Normal3s.load(&mut loadfn, "glNormal3s");
         storage::Normal3sv.load(&mut loadfn, "glNormal3sv");
         storage::NormalPointer.load(&mut loadfn, "glNormalPointer");
         storage::Ortho.load(&mut loadfn, "glOrtho");
         storage::PassThrough.load(&mut loadfn, "glPassThrough");
         storage::PixelMapfv.load(&mut loadfn, "glPixelMapfv");
         storage::PixelMapuiv.load(&mut loadfn, "glPixelMapuiv");
         storage::PixelMapusv.load(&mut loadfn, "glPixelMapusv");
         storage::PixelStoref.load(&mut loadfn, "glPixelStoref");
         storage::PixelStorei.load(&mut loadfn, "glPixelStorei");
         storage::PixelTransferf.load(&mut loadfn, "glPixelTransferf");
         storage::PixelTransferi.load(&mut loadfn, "glPixelTransferi");
         storage::PixelZoom.load(&mut loadfn, "glPixelZoom");
         storage::PointParameterf.load(&mut loadfn, "glPointParameterf");
         storage::PointParameterfv.load(&mut loadfn, "glPointParameterfv");
         storage::PointParameteri.load(&mut loadfn, "glPointParameteri");
         storage::PointParameteriv.load(&mut loadfn, "glPointParameteriv");
         storage::PointSize.load(&mut loadfn, "glPointSize");
         storage::PolygonMode.load(&mut loadfn, "glPolygonMode");
         storage::PolygonOffset.load(&mut loadfn, "glPolygonOffset");
         storage::PolygonStipple.load(&mut loadfn, "glPolygonStipple");
         storage::PopAttrib.load(&mut loadfn, "glPopAttrib");
         storage::PopClientAttrib.load(&mut loadfn, "glPopClientAttrib");
         storage::PopMatrix.load(&mut loadfn, "glPopMatrix");
         storage::PopName.load(&mut loadfn, "glPopName");
         storage::PrioritizeTextures.load(&mut loadfn, "glPrioritizeTextures");
         storage::PushAttrib.load(&mut loadfn, "glPushAttrib");
         storage::PushClientAttrib.load(&mut loadfn, "glPushClientAttrib");
         storage::PushMatrix.load(&mut loadfn, "glPushMatrix");
         storage::PushName.load(&mut loadfn, "glPushName");
         storage::RasterPos2d.load(&mut loadfn, "glRasterPos2d");
         storage::RasterPos2dv.load(&mut loadfn, "glRasterPos2dv");
         storage::RasterPos2f.load(&mut loadfn, "glRasterPos2f");
         storage::RasterPos2fv.load(&mut loadfn, "glRasterPos2fv");
         storage::RasterPos2i.load(&mut loadfn, "glRasterPos2i");
         storage::RasterPos2iv.load(&mut loadfn, "glRasterPos2iv");
         storage::RasterPos2s.load(&mut loadfn, "glRasterPos2s");
         storage::RasterPos2sv.load(&mut loadfn, "glRasterPos2sv");
         storage::RasterPos3d.load(&mut loadfn, "glRasterPos3d");
         storage::RasterPos3dv.load(&mut loadfn, "glRasterPos3dv");
         storage::RasterPos3f.load(&mut loadfn, "glRasterPos3f");
         storage::RasterPos3fv.load(&mut loadfn, "glRasterPos3fv");
         storage::RasterPos3i.load(&mut loadfn, "glRasterPos3i");
         storage::RasterPos3iv.load(&mut loadfn, "glRasterPos3iv");
         storage::RasterPos3s.load(&mut loadfn, "glRasterPos3s");
         storage::RasterPos3sv.load(&mut loadfn, "glRasterPos3sv");
         storage::RasterPos4d.load(&mut loadfn, "glRasterPos4d");
         storage::RasterPos4dv.load(&mut loadfn, "glRasterPos4dv");
         storage::RasterPos4f.load(&mut loadfn, "glRasterPos4f");
         storage::RasterPos4fv.load(&mut loadfn, "glRasterPos4fv");
         storage::RasterPos4i.load(&mut loadfn, "glRasterPos4i");
         storage::RasterPos4iv.load(&mut loadfn, "glRasterPos4iv");
         storage::RasterPos4s.load(&mut loadfn, "glRasterPos4s");
         storage::RasterPos4sv.load(&mut loadfn, "glRasterPos4sv");
         storage::ReadBuffer.load(&mut loadfn, "glReadBuffer");
         storage::ReadPixels.load(&mut loadfn, "glReadPixels");
         storage::Rectd.load(&mut loadfn, "glRectd");
         storage::Rectdv.load(&mut loadfn, "glRectdv");
         storage::Rectf.load(&mut loadfn, "glRectf");
         storage::Rectfv.load(&mut loadfn, "glRectfv");
         storage::Recti.load(&mut loadfn, "glRecti");
         storage::Rectiv.load(&mut loadfn, "glRectiv");
         storage::Rects.load(&mut loadfn, "glRects");
         storage::Rectsv.load(&mut loadfn, "glRectsv");
         storage::RenderMode.load(&mut loadfn, "glRenderMode");
         storage::Rotated.load(&mut loadfn, "glRotated");
         storage::Rotatef.load(&mut loadfn, "glRotatef");
         storage::SampleCoverage.load(&mut loadfn, "glSampleCoverage");
         storage::Scaled.load(&mut loadfn, "glScaled");
         storage::Scalef.load(&mut loadfn, "glScalef");
         storage::Scissor.load(&mut loadfn, "glScissor");
         storage::SecondaryColor3b.load(&mut loadfn, "glSecondaryColor3b");
         storage::SecondaryColor3bv.load(&mut loadfn, "glSecondaryColor3bv");
         storage::SecondaryColor3d.load(&mut loadfn, "glSecondaryColor3d");
         storage::SecondaryColor3dv.load(&mut loadfn, "glSecondaryColor3dv");
         storage::SecondaryColor3f.load(&mut loadfn, "glSecondaryColor3f");
         storage::SecondaryColor3fv.load(&mut loadfn, "glSecondaryColor3fv");
         storage::SecondaryColor3i.load(&mut loadfn, "glSecondaryColor3i");
         storage::SecondaryColor3iv.load(&mut loadfn, "glSecondaryColor3iv");
         storage::SecondaryColor3s.load(&mut loadfn, "glSecondaryColor3s");
         storage::SecondaryColor3sv.load(&mut loadfn, "glSecondaryColor3sv");
         storage::SecondaryColor3ub.load(&mut loadfn, "glSecondaryColor3ub");
         storage::SecondaryColor3ubv.load(&mut loadfn, "glSecondaryColor3ubv");
         storage::SecondaryColor3ui.load(&mut loadfn, "glSecondaryColor3ui");
         storage::SecondaryColor3uiv.load(&mut loadfn, "glSecondaryColor3uiv");
         storage::SecondaryColor3us.load(&mut loadfn, "glSecondaryColor3us");
         storage::SecondaryColor3usv.load(&mut loadfn, "glSecondaryColor3usv");
         storage::SecondaryColorPointer.load(&mut loadfn, "glSecondaryColorPointer");
         storage::SelectBuffer.load(&mut loadfn, "glSelectBuffer");
         storage::ShadeModel.load(&mut loadfn, "glShadeModel");
         storage::ShaderSource.load(&mut loadfn, "glShaderSource");
         storage::StencilFunc.load(&mut loadfn, "glStencilFunc");
         storage::StencilFuncSeparate.load(&mut loadfn, "glStencilFuncSeparate");
         storage::StencilMask.load(&mut loadfn, "glStencilMask");
         storage::StencilMaskSeparate.load(&mut loadfn, "glStencilMaskSeparate");
         storage::StencilOp.load(&mut loadfn, "glStencilOp");
         storage::StencilOpSeparate.load(&mut loadfn, "glStencilOpSeparate");
         storage::TexCoord1d.load(&mut loadfn, "glTexCoord1d");
         storage::TexCoord1dv.load(&mut loadfn, "glTexCoord1dv");
         storage::TexCoord1f.load(&mut loadfn, "glTexCoord1f");
         storage::TexCoord1fv.load(&mut loadfn, "glTexCoord1fv");
         storage::TexCoord1i.load(&mut loadfn, "glTexCoord1i");
         storage::TexCoord1iv.load(&mut loadfn, "glTexCoord1iv");
         storage::TexCoord1s.load(&mut loadfn, "glTexCoord1s");
         storage::TexCoord1sv.load(&mut loadfn, "glTexCoord1sv");
         storage::TexCoord2d.load(&mut loadfn, "glTexCoord2d");
         storage::TexCoord2dv.load(&mut loadfn, "glTexCoord2dv");
         storage::TexCoord2f.load(&mut loadfn, "glTexCoord2f");
         storage::TexCoord2fv.load(&mut loadfn, "glTexCoord2fv");
         storage::TexCoord2i.load(&mut loadfn, "glTexCoord2i");
         storage::TexCoord2iv.load(&mut loadfn, "glTexCoord2iv");
         storage::TexCoord2s.load(&mut loadfn, "glTexCoord2s");
         storage::TexCoord2sv.load(&mut loadfn, "glTexCoord2sv");
         storage::TexCoord3d.load(&mut loadfn, "glTexCoord3d");
         storage::TexCoord3dv.load(&mut loadfn, "glTexCoord3dv");
         storage::TexCoord3f.load(&mut loadfn, "glTexCoord3f");
         storage::TexCoord3fv.load(&mut loadfn, "glTexCoord3fv");
         storage::TexCoord3i.load(&mut loadfn, "glTexCoord3i");
         storage::TexCoord3iv.load(&mut loadfn, "glTexCoord3iv");
         storage::TexCoord3s.load(&mut loadfn, "glTexCoord3s");
         storage::TexCoord3sv.load(&mut loadfn, "glTexCoord3sv");
         storage::TexCoord4d.load(&mut loadfn, "glTexCoord4d");
         storage::TexCoord4dv.load(&mut loadfn, "glTexCoord4dv");
         storage::TexCoord4f.load(&mut loadfn, "glTexCoord4f");
         storage::TexCoord4fv.load(&mut loadfn, "glTexCoord4fv");
         storage::TexCoord4i.load(&mut loadfn, "glTexCoord4i");
         storage::TexCoord4iv.load(&mut loadfn, "glTexCoord4iv");
         storage::TexCoord4s.load(&mut loadfn, "glTexCoord4s");
         storage::TexCoord4sv.load(&mut loadfn, "glTexCoord4sv");
         storage::TexCoordPointer.load(&mut loadfn, "glTexCoordPointer");
         storage::TexEnvf.load(&mut loadfn, "glTexEnvf");
         storage::TexEnvfv.load(&mut loadfn, "glTexEnvfv");
         storage::TexEnvi.load(&mut loadfn, "glTexEnvi");
         storage::TexEnviv.load(&mut loadfn, "glTexEnviv");
         storage::TexGend.load(&mut loadfn, "glTexGend");
         storage::TexGendv.load(&mut loadfn, "glTexGendv");
         storage::TexGenf.load(&mut loadfn, "glTexGenf");
         storage::TexGenfv.load(&mut loadfn, "glTexGenfv");
         storage::TexGeni.load(&mut loadfn, "glTexGeni");
         storage::TexGeniv.load(&mut loadfn, "glTexGeniv");
         storage::TexImage1D.load(&mut loadfn, "glTexImage1D");
         storage::TexImage2D.load(&mut loadfn, "glTexImage2D");
         storage::TexImage3D.load(&mut loadfn, "glTexImage3D");
         storage::TexParameterf.load(&mut loadfn, "glTexParameterf");
         storage::TexParameterfv.load(&mut loadfn, "glTexParameterfv");
         storage::TexParameteri.load(&mut loadfn, "glTexParameteri");
         storage::TexParameteriv.load(&mut loadfn, "glTexParameteriv");
         storage::TexSubImage1D.load(&mut loadfn, "glTexSubImage1D");
         storage::TexSubImage2D.load(&mut loadfn, "glTexSubImage2D");
         storage::TexSubImage3D.load(&mut loadfn, "glTexSubImage3D");
         storage::Translated.load(&mut loadfn, "glTranslated");
         storage::Translatef.load(&mut loadfn, "glTranslatef");
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
         storage::UnmapBuffer.load(&mut loadfn, "glUnmapBuffer");
         storage::UseProgram.load(&mut loadfn, "glUseProgram");
         storage::ValidateProgram.load(&mut loadfn, "glValidateProgram");
         storage::Vertex2d.load(&mut loadfn, "glVertex2d");
         storage::Vertex2dv.load(&mut loadfn, "glVertex2dv");
         storage::Vertex2f.load(&mut loadfn, "glVertex2f");
         storage::Vertex2fv.load(&mut loadfn, "glVertex2fv");
         storage::Vertex2i.load(&mut loadfn, "glVertex2i");
         storage::Vertex2iv.load(&mut loadfn, "glVertex2iv");
         storage::Vertex2s.load(&mut loadfn, "glVertex2s");
         storage::Vertex2sv.load(&mut loadfn, "glVertex2sv");
         storage::Vertex3d.load(&mut loadfn, "glVertex3d");
         storage::Vertex3dv.load(&mut loadfn, "glVertex3dv");
         storage::Vertex3f.load(&mut loadfn, "glVertex3f");
         storage::Vertex3fv.load(&mut loadfn, "glVertex3fv");
         storage::Vertex3i.load(&mut loadfn, "glVertex3i");
         storage::Vertex3iv.load(&mut loadfn, "glVertex3iv");
         storage::Vertex3s.load(&mut loadfn, "glVertex3s");
         storage::Vertex3sv.load(&mut loadfn, "glVertex3sv");
         storage::Vertex4d.load(&mut loadfn, "glVertex4d");
         storage::Vertex4dv.load(&mut loadfn, "glVertex4dv");
         storage::Vertex4f.load(&mut loadfn, "glVertex4f");
         storage::Vertex4fv.load(&mut loadfn, "glVertex4fv");
         storage::Vertex4i.load(&mut loadfn, "glVertex4i");
         storage::Vertex4iv.load(&mut loadfn, "glVertex4iv");
         storage::Vertex4s.load(&mut loadfn, "glVertex4s");
         storage::Vertex4sv.load(&mut loadfn, "glVertex4sv");
         storage::VertexAttrib1d.load(&mut loadfn, "glVertexAttrib1d");
         storage::VertexAttrib1dv.load(&mut loadfn, "glVertexAttrib1dv");
         storage::VertexAttrib1f.load(&mut loadfn, "glVertexAttrib1f");
         storage::VertexAttrib1fv.load(&mut loadfn, "glVertexAttrib1fv");
         storage::VertexAttrib1s.load(&mut loadfn, "glVertexAttrib1s");
         storage::VertexAttrib1sv.load(&mut loadfn, "glVertexAttrib1sv");
         storage::VertexAttrib2d.load(&mut loadfn, "glVertexAttrib2d");
         storage::VertexAttrib2dv.load(&mut loadfn, "glVertexAttrib2dv");
         storage::VertexAttrib2f.load(&mut loadfn, "glVertexAttrib2f");
         storage::VertexAttrib2fv.load(&mut loadfn, "glVertexAttrib2fv");
         storage::VertexAttrib2s.load(&mut loadfn, "glVertexAttrib2s");
         storage::VertexAttrib2sv.load(&mut loadfn, "glVertexAttrib2sv");
         storage::VertexAttrib3d.load(&mut loadfn, "glVertexAttrib3d");
         storage::VertexAttrib3dv.load(&mut loadfn, "glVertexAttrib3dv");
         storage::VertexAttrib3f.load(&mut loadfn, "glVertexAttrib3f");
         storage::VertexAttrib3fv.load(&mut loadfn, "glVertexAttrib3fv");
         storage::VertexAttrib3s.load(&mut loadfn, "glVertexAttrib3s");
         storage::VertexAttrib3sv.load(&mut loadfn, "glVertexAttrib3sv");
         storage::VertexAttrib4Nbv.load(&mut loadfn, "glVertexAttrib4Nbv");
         storage::VertexAttrib4Niv.load(&mut loadfn, "glVertexAttrib4Niv");
         storage::VertexAttrib4Nsv.load(&mut loadfn, "glVertexAttrib4Nsv");
         storage::VertexAttrib4Nub.load(&mut loadfn, "glVertexAttrib4Nub");
         storage::VertexAttrib4Nubv.load(&mut loadfn, "glVertexAttrib4Nubv");
         storage::VertexAttrib4Nuiv.load(&mut loadfn, "glVertexAttrib4Nuiv");
         storage::VertexAttrib4Nusv.load(&mut loadfn, "glVertexAttrib4Nusv");
         storage::VertexAttrib4bv.load(&mut loadfn, "glVertexAttrib4bv");
         storage::VertexAttrib4d.load(&mut loadfn, "glVertexAttrib4d");
         storage::VertexAttrib4dv.load(&mut loadfn, "glVertexAttrib4dv");
         storage::VertexAttrib4f.load(&mut loadfn, "glVertexAttrib4f");
         storage::VertexAttrib4fv.load(&mut loadfn, "glVertexAttrib4fv");
         storage::VertexAttrib4iv.load(&mut loadfn, "glVertexAttrib4iv");
         storage::VertexAttrib4s.load(&mut loadfn, "glVertexAttrib4s");
         storage::VertexAttrib4sv.load(&mut loadfn, "glVertexAttrib4sv");
         storage::VertexAttrib4ubv.load(&mut loadfn, "glVertexAttrib4ubv");
         storage::VertexAttrib4uiv.load(&mut loadfn, "glVertexAttrib4uiv");
         storage::VertexAttrib4usv.load(&mut loadfn, "glVertexAttrib4usv");
         storage::VertexAttribPointer.load(&mut loadfn, "glVertexAttribPointer");
         storage::VertexPointer.load(&mut loadfn, "glVertexPointer");
         storage::Viewport.load(&mut loadfn, "glViewport");
         storage::WindowPos2d.load(&mut loadfn, "glWindowPos2d");
         storage::WindowPos2dv.load(&mut loadfn, "glWindowPos2dv");
         storage::WindowPos2f.load(&mut loadfn, "glWindowPos2f");
         storage::WindowPos2fv.load(&mut loadfn, "glWindowPos2fv");
         storage::WindowPos2i.load(&mut loadfn, "glWindowPos2i");
         storage::WindowPos2iv.load(&mut loadfn, "glWindowPos2iv");
         storage::WindowPos2s.load(&mut loadfn, "glWindowPos2s");
         storage::WindowPos2sv.load(&mut loadfn, "glWindowPos2sv");
         storage::WindowPos3d.load(&mut loadfn, "glWindowPos3d");
         storage::WindowPos3dv.load(&mut loadfn, "glWindowPos3dv");
         storage::WindowPos3f.load(&mut loadfn, "glWindowPos3f");
         storage::WindowPos3fv.load(&mut loadfn, "glWindowPos3fv");
         storage::WindowPos3i.load(&mut loadfn, "glWindowPos3i");
         storage::WindowPos3iv.load(&mut loadfn, "glWindowPos3iv");
         storage::WindowPos3s.load(&mut loadfn, "glWindowPos3s");
         storage::WindowPos3sv.load(&mut loadfn, "glWindowPos3sv");

    }
}

