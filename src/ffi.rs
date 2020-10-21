use super::*;
use std::os::raw::c_int;

#[repr(C)]
pub struct UserData {
    private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn new_quadtree() -> *mut Quadtree {
    let obj = Quadtree::new(&Rectangle::new(10., 10., 10., 10.));
    let boxed = Box::new(obj);

    Box::into_raw(boxed)
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn quadtree_insert_point(
    qt: *mut Quadtree,
    x: f32,
    y: f32,
    data: *const UserData,
) -> c_int {
    // TODO: Make this an enum for return error
    if qt.is_null() {
        return -1;
    }

    let qt = &mut *qt;
    qt.insert_point(&Point { x, y, data })
        .map(|_| 0)
        .unwrap_or(-2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_api() {
        let qt = new_quadtree();
        assert!(!qt.is_null());

        let result = unsafe { quadtree_insert_point(qt, 1.0, 2.0, std::ptr::null()) };
        assert_eq!(result, 0);
    }
}
