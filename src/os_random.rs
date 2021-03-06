#![allow(bad_style)]

use core::convert::TryInto;

/// Fills a byte buffer with random bytes from an OS-specific source.
pub fn fill_byte_buffer_from_os_random(buf: &mut [u8]) -> Result<(), u32> {
  #[cfg(windows)]
  {
    type BCRYPT_ALG_HANDLE = *mut core::ffi::c_void;
    const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;
    const STATUS_SUCCESS: i32 = 0x00000000;
    #[link(name = "Bcrypt")]
    extern "system" {
      fn BCryptGenRandom(hAlgorithm: BCRYPT_ALG_HANDLE, pbBuffer: *mut u8, cbBuffer: u32, dwFlags: u32) -> i32;
    }
    #[link(name = "Kernel32")]
    extern "system" {
      fn GetLastError() -> u32;
    }
    unsafe {
      let status = BCryptGenRandom(0 as _, buf.as_mut_ptr(), buf.len().try_into().unwrap(), BCRYPT_USE_SYSTEM_PREFERRED_RNG);
      if status == STATUS_SUCCESS {
        Ok(())
      } else {
        Err(GetLastError())
      }
    }
  }
  #[cfg(unix)]
  {
    #[cfg(target_pointer_width = "16")]
    compile_error!("16-bit systems not supported");
    extern "C" {
      fn getrandom(buf: *mut u8, buf_len: usize, flags: u32) -> isize;
    }
    unsafe {
      let status = getrandom(buf.as_mut_ptr(), buf.len().try_into().unwrap(), 0);
      if status != -1 {
        Ok(())
      } else {
        Err(u32::MAX)
      }
    }
  }
  #[cfg(not(any(windows, unix)))]
  {
    compile_error!("The `os_random` feature requires windows or unix!")
  }
}
