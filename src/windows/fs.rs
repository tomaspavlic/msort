use anyhow::Context;
use std::path::Path;
use widestring::U16CString;
use winapi::{
    ctypes::c_void,
    shared::minwindef::{BOOL, LPVOID},
    um::{
        winbase::{self, LPPROGRESS_ROUTINE},
        winnt::LARGE_INTEGER,
    },
};

pub fn copy<P, F>(from: P, to: P, progress_callback: F) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    F: Fn(i32) + 'static,
{
    let from = U16CString::from_str(from.as_ref().to_str().context("failed converting path")?)?;
    let to = U16CString::from_str(to.as_ref().to_str().context("failed convert path")?)?;
    let c = &mut 0 as *mut BOOL;
    let context = &progress_callback as *const _ as LPVOID;
    let callback: LPPROGRESS_ROUTINE = Some(invoke::<F>);
    let r = unsafe { winbase::CopyFileExW(from.as_ptr(), to.as_ptr(), callback, context, c, 0) };
    if r != 1 {
        anyhow::bail!("error copying file");
    }

    Ok(())
}

unsafe extern "system" fn invoke<F>(
    _total_file_size: LARGE_INTEGER,
    total_bytes_transferred: LARGE_INTEGER,
    _stream_size: LARGE_INTEGER,
    _stream_bytes_transferred: LARGE_INTEGER,
    _dw_stream_number: u32,
    _dw_callback_reason: u32,
    _hsource_file: *mut c_void,
    _hdestination_file: *mut c_void,
    lpdata: *mut c_void,
) -> u32
where
    F: Fn(i32) + 'static,
{
    let callback = unsafe { &*(lpdata as *const F) };
    let transferred = unsafe { total_bytes_transferred.s().LowPart };
    let x = transferred.try_into().unwrap();
    (callback)(x);

    return 0;
}
