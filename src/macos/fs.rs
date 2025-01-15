use crate::macos::bind::{
    _copyfile_state, copyfile, copyfile_state_alloc, copyfile_state_get, copyfile_state_set,
    COPYFILE_ALL, COPYFILE_COPY_DATA, COPYFILE_PROGRESS, COPYFILE_STATE_COPIED,
    COPYFILE_STATE_STATUS_CB, COPYFILE_STATE_STATUS_CTX,
};
use anyhow::{bail, Context};
use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_void},
    path::Path,
};

pub fn copy<P, F>(from: P, to: P, progress_callback: F) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    F: Fn(i32) + 'static,
{
    let state = unsafe { copyfile_state_alloc() };
    let from = CString::new(from.as_ref().to_str().context("invalid path")?)?;
    let to = CString::new(to.as_ref().to_str().context("invalid path")?)?;

    unsafe {
        copyfile_state_set(
            state,
            COPYFILE_STATE_STATUS_CTX,
            &progress_callback as *const _ as *const c_void,
        );
        copyfile_state_set(
            state,
            COPYFILE_STATE_STATUS_CB,
            copyfile_callback::<F> as *const c_void,
        );
        let result_code = copyfile(from.as_ptr(), to.as_ptr(), state, COPYFILE_ALL);
        if result_code == 0 {
            Ok(())
        } else {
            bail!("failed copying file")
        }
    }
}

fn copyfile_callback<F>(
    event: u32,
    stage: u32,
    state: *mut _copyfile_state,
    _from: *const c_char,
    _to: *const c_char,
    context: *const c_void,
) -> u32
where
    F: Fn(i32) + 'static,
{
    if event == COPYFILE_COPY_DATA && stage == COPYFILE_PROGRESS {
        let callback = unsafe { &*(context as *const F) };
        let mut copied_bytes: c_int = 0;
        let return_code = unsafe {
            copyfile_state_get(
                state,
                COPYFILE_STATE_COPIED,
                &mut copied_bytes as *mut c_int as *mut c_void,
            )
        };

        if return_code == 0 {
            (callback)(copied_bytes);
        }
    }

    0
}
