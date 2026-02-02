use core::mem::MaybeUninit;

use pinocchio::error::ProgramError;

use crate::errors::CustomError;

/// Trait for structs which are of u8 byte arrays.
/// Used instead of repr(packed) for stability and predictability (?)
/// 
/// # SAFETY
/// - Struct must be constructed by u8 bytes
/// - All data must be initialized
pub unsafe trait Transmutable { 
    const LEN: usize;

    #[inline(always)]
    fn as_bytes(&self) -> &[u8] where Self: Sized {
        as_bytes(self)
    }

    #[inline(always)]
    fn from_bytes(data: &[u8]) -> Result<&Self, ProgramError> where Self: Sized {
        from_bytes(data)
    }
}

/// Forms a slice from t of type T
#[inline(always)]
pub fn as_bytes<T: Transmutable>(t: &T) -> &[u8] {

    // SAFETY: T is of trait Transmutable
    unsafe {
        core::slice::from_raw_parts(
            t as *const T as *const u8, 
            size_of::<T>())
    }
}

#[inline(always)]
pub fn from_bytes<T: Transmutable>(data: &[u8]) -> Result<&T, ProgramError> {
    if data.len() != size_of::<T>() {
        return Err(CustomError::TransmutableError.into()); 
    }

    // SAFETY:
    // - data's length is the same as T
    // - Data is of trait Transmutable
    unsafe { Ok(from_bytes_unchecked(data)) }
}

/// Get data as a reference of type T
///
/// # SAFETY
/// - 'data' must be a valid representation of T
#[inline(always)]
pub unsafe fn from_bytes_unchecked<T: Transmutable>(data: &[u8]) -> &T {
    &*(data.as_ptr() as *const T)
}

/// Write bytes from dst to src
#[inline(always)]
pub fn write_bytes(dst: &mut [MaybeUninit<u8>], src: &[u8]) {

    // SAFETY:
    // - Pointers are of alignment 1,
    // - the length will not exceed either pointers length
    unsafe {
        core::ptr::copy_nonoverlapping(
            src.as_ptr(), 
            dst.as_mut_ptr() as *mut u8, 
            dst.len().min(src.len())
        );
    }
}
