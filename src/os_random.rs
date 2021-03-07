#![cfg(feature = "os_random")]
#![allow(bad_style)]

#[allow(unused_imports)]
use core::convert::TryInto;

/// Fills a byte buffer with random bytes from an OS-specific source.
///
/// This is intended for use in obtaining small seed slices (<=64 bytes) to
/// initialize a PRNG with. Despite that intent, you *can* attempt to fill a
/// buffer of any size. The function will automatically chunk up the buffer
/// appropriately so that each individual randomness request to the OS doesn't
/// overwhelm the OS call.
///
/// It's possible that the OS randomness call can fail. In this case, the
/// function will automatically attempt to retry up to 10 times. If failure
/// persists, then this function will give up and pass the `Err` back to you.
/// When this happens, some unknown amount of the buffer *might* have been
/// filled with randomness before things broke.
///
/// #### The OS Call Used Varies By Target
/// * **Windows:** [BCryptGenRandom][bcrypt]. On failure, the `Err` holds the
///   `GetLastError` value.
/// * **Unix:** [getrandom][gr]. On failure, the `Err` value will be `u32::MAX`.
/// * **Other:** This function will **fail to link** if you don't have either
///   `windows` or `unix` configured.
///
/// Because cargo does not handle target-conditional features very well, this
/// function always exists as long as the `os_random` crate feature is enabled.
/// However, if you do actually call this function when building for a target
/// other than `windows` or `unix` (including MacOS), then you'll get a linker
/// error.
///
/// In other words, you can safely leave the `os_random` feature on all the time
/// and still build the crate anywhere, as long as you don't *actually* call
/// this function outside of `windows` or `unix`.
///
/// [bcrypt]:
/// https://docs.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptgenrandom
/// [gr]: https://man7.org/linux/man-pages/man2/getrandom.2.html
pub fn fill_byte_buffer_from_os_random(buf: &mut [u8]) -> Result<(), u32> {
  #[cfg(target_pointer_width = "16")]
  compile_error!("16-bit systems not supported");
  #[cfg(windows)]
  {
    type BCRYPT_ALG_HANDLE = *mut core::ffi::c_void;
    const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;
    const STATUS_SUCCESS: i32 = 0x00000000;
    #[link(name = "Bcrypt")]
    extern "system" {
      /// https://docs.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptgenrandom
      fn BCryptGenRandom(
        hAlgorithm: BCRYPT_ALG_HANDLE, pbBuffer: *mut u8, cbBuffer: u32, dwFlags: u32,
      ) -> i32;
    }
    #[link(name = "Kernel32")]
    extern "system" {
      fn GetLastError() -> u32;
    }
    //
    for chunk in buf.chunks_mut(u32::MAX as usize) {
      let mut chunk_retries = 10;
      'bcrypt: loop {
        let status = unsafe {
          BCryptGenRandom(
            0 as _,
            chunk.as_mut_ptr(),
            chunk.len().try_into().unwrap(),
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
          )
        };
        if status == STATUS_SUCCESS {
          break 'bcrypt;
        }
        chunk_retries -= 1;
        if chunk_retries == 0 {
          return Err(unsafe { GetLastError() });
        }
      }
    }
    Ok(())
  }
  #[cfg(unix)]
  {
    #[link(name = "c")]
    extern "C" {
      /// https://man7.org/linux/man-pages/man2/getrandom.2.html
      fn getrandom(buf: *mut u8, buf_len: usize, flags: u32) -> isize;
    }
    const MAX_URANDOM_REQUEST_SIZE: usize = 33554431;
    assert!(MAX_URANDOM_REQUEST_SIZE <= u32::MAX as usize);
    for mut chunk in buf.chunks_mut(MAX_URANDOM_REQUEST_SIZE) {
      let mut chunk_retries = 10;
      'getrandom: loop {
        let chunk_len_u32 = chunk.len().try_into().unwrap();
        let chunk_p = chunk.as_mut_ptr();
        let bytes_randomized = unsafe { getrandom(chunk_p, chunk_len_u32, 0) };
        if bytes_randomized < 0 {
          chunk_retries -= 1;
          if chunk_retries == 0 {
            return Err(u32::MAX);
          }
        } else {
          chunk = &mut chunk[bytes_randomized as usize..];
          if chunk.is_empty() {
            break 'getrandom;
          }
        }
      }
    }
    Ok(())
  }
  #[cfg(not(any(windows, unix)))]
  {
    extern "C" {
      fn the_os_random_feature_requires_either_windows_or_unix();
    }
    unsafe { the_os_random_feature_requires_either_windows_or_unix() };
    Ok(())
  }
}
