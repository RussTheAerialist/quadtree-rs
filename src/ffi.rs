use super::*;
use std::os::raw::{c_int, c_void};
pub struct UserData {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn quadtree_new(x: f32, y: f32, w: f32, h: f32, capacity: u8) -> *mut Quadtree {
    let capacity = if capacity == 0 { 10u8 } else { capacity };

    let obj = Quadtree::with_capacity(&Rectangle::new(x, y, w, h), capacity);
    let boxed = Box::new(obj);

    Box::into_raw(boxed)
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn quadtree_free(qt: *mut Quadtree) {
    let _ = Box::from_raw(qt);
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn quadtree_insert_point(
    qt: *mut Quadtree,
    x: f32,
    y: f32,
    data: *const c_void,
) -> c_int {
    // TODO: Make this an enum for return error
    if qt.is_null() {
        return -1;
    }

    let qt = &mut *qt;
    qt.insert(&Point { x, y, data: data as *const UserData }).map(|_| 0).unwrap_or(-2)
}

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn quadtree_query(
    qt: *const Quadtree,
    x: f32,
    y: f32,
    r: f32,
    count: &mut usize,
) -> *const c_void {
    if qt.is_null() {
        *count = 0;
        return std::ptr::null_mut();
    }

    let qt = &*qt;
    let point = Point::new(x, y);
    let mut points: Vec<*const UserData> =
        qt.query(&point, r).into_iter().map(|p| p.data).collect();

    points.shrink_to_fit();
    let ptr = points.as_ptr();
    *count = points.len();

    std::mem::forget(points);

    ptr as *const c_void
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_api() {
        let qt = quadtree_new(10., 10., 10., 10., 10);
        assert!(!qt.is_null());

        let result = unsafe { quadtree_insert_point(qt, 1.0, 2.0, std::ptr::null()) };
        assert_eq!(result, 0);
        let mut count = 0;
        let result = unsafe { quadtree_query(qt, 1., 1., 5., &mut count) };
        assert_eq!(1, count);
        let result = unsafe { Vec::from_raw_parts(result as *mut *const UserData, count, count) };
        let value = result[0];
        assert!(value.is_null());

        unsafe {
            quadtree_free(qt);
        }
    }
}
