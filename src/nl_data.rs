// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

#[derive(Debug)]
#[repr(C)]
pub struct NlData {
    essid: *const c_char,
    signal: c_int,
}

#[derive(Debug)]
pub struct Data {
    essid: Option<String>,
    signal: Option<i32>,
}

#[link(name = "nl_data", kind = "static")]
extern "C" {
    pub fn get_data() -> *const NlData;
}

pub fn data() -> Option<Data> {
    unsafe {
        let nl_data = get_data();
        let signal_ptr = (*nl_data).signal;
        let essid_ptr = (*nl_data).essid;
        let signal = if signal_ptr == -1 {
            None
        } else {
            Some(signal_ptr)
        };
        let essid = if essid_ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(essid_ptr).to_string_lossy().into_owned())
        };
        return if signal.is_none() && essid.is_none() {
            None
        } else {
            Some(Data { signal, essid })
        };
    }
}