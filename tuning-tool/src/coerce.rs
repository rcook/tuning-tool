use midly::num::u7;
use tuning_tool_lib::U7;

pub(crate) fn unsafe_coerce_slice_to_u7_slice<U: U7>(slice: &[U]) -> &[u7] {
    let u8_slice = unsafe_coerce_slice_to_u8_slice(slice);
    let u7_slice = unsafe { u7::slice_from_int_unchecked(u8_slice) };
    u7_slice
}

pub(crate) fn unsafe_coerce_slice_to_u8_slice<U: U7>(slice: &[U]) -> &[u8] {
    unsafe { &*(slice as *const [U] as *const [u8]) }
}

pub(crate) fn really_really_unsafe_coerce_slice<F: U7, T: U7>(slice: &[F]) -> &[T] {
    unsafe { &*(slice as *const [F] as *const [T]) }
}
