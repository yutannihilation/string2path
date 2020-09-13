use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn glyph2digit(c_glyph: *const c_char, c_ttf_file: *const c_char) -> u32 {
    let glyph = unsafe { CStr::from_ptr(c_glyph).to_string_lossy().into_owned() };

    let ttf_file = unsafe { CStr::from_ptr(c_ttf_file).to_string_lossy().into_owned() };
    let font = {
        if let Ok(data) = std::fs::read(ttf_file) {
            rusttype::Font::try_from_vec(data).unwrap()
        } else {
            return 0;
        }
    };

    let c = glyph.chars().next().unwrap();
    font.glyph(c).id().0.into()
}
