// Copyright (c) 2014 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::ptr;

//
// Callback structure used for asynchronous continuation of quota requests.
//
#[repr(C)]
pub struct _cef_quota_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Continue the quota request. If |allow| is true (1) the request will be
  // allowed. Otherwise, the request will be denied.
  //
  pub cont: Option<extern "C" fn(this: *mut cef_quota_callback_t,
      allow: libc::c_int) -> ()>,

  //
  // Cancel the quota request.
  //
  pub cancel: Option<extern "C" fn(this: *mut cef_quota_callback_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_quota_callback_t = _cef_quota_callback_t;


//
// Callback structure used for asynchronous continuation of quota requests.
//
pub struct CefQuotaCallback {
  c_object: *mut cef_quota_callback_t,
}

impl Clone for CefQuotaCallback {
  fn clone(&self) -> CefQuotaCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefQuotaCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefQuotaCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefQuotaCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_quota_callback_t) -> CefQuotaCallback {
    CefQuotaCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_quota_callback_t) -> CefQuotaCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefQuotaCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_quota_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_quota_callback_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Continue the quota request. If |allow| is true (1) the request will be
  // allowed. Otherwise, the request will be denied.
  //
  pub fn cont(&self, allow: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object,
          CefWrap::to_c(allow)))
    }
  }

  //
  // Cancel the quota request.
  //
  pub fn cancel(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cancel.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_quota_callback_t> for CefQuotaCallback {
  fn to_c(rust_object: CefQuotaCallback) -> *mut cef_quota_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_quota_callback_t) -> CefQuotaCallback {
    CefQuotaCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_quota_callback_t> for Option<CefQuotaCallback> {
  fn to_c(rust_object: Option<CefQuotaCallback>) -> *mut cef_quota_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_quota_callback_t) -> Option<CefQuotaCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefQuotaCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Callback structure used for asynchronous continuation of url requests when
// invalid SSL certificates are encountered.
//
#[repr(C)]
pub struct _cef_allow_certificate_error_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Continue the url request. If |allow| is true (1) the request will be
  // continued. Otherwise, the request will be canceled.
  //
  pub cont: Option<extern "C" fn(
      this: *mut cef_allow_certificate_error_callback_t,
      allow: libc::c_int) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_allow_certificate_error_callback_t = _cef_allow_certificate_error_callback_t;


//
// Callback structure used for asynchronous continuation of url requests when
// invalid SSL certificates are encountered.
//
pub struct CefAllowCertificateErrorCallback {
  c_object: *mut cef_allow_certificate_error_callback_t,
}

impl Clone for CefAllowCertificateErrorCallback {
  fn clone(&self) -> CefAllowCertificateErrorCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefAllowCertificateErrorCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefAllowCertificateErrorCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefAllowCertificateErrorCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_allow_certificate_error_callback_t) -> CefAllowCertificateErrorCallback {
    CefAllowCertificateErrorCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_allow_certificate_error_callback_t) -> CefAllowCertificateErrorCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefAllowCertificateErrorCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_allow_certificate_error_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_allow_certificate_error_callback_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Continue the url request. If |allow| is true (1) the request will be
  // continued. Otherwise, the request will be canceled.
  //
  pub fn cont(&self, allow: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object,
          CefWrap::to_c(allow)))
    }
  }
} 

impl CefWrap<*mut cef_allow_certificate_error_callback_t> for CefAllowCertificateErrorCallback {
  fn to_c(rust_object: CefAllowCertificateErrorCallback) -> *mut cef_allow_certificate_error_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_allow_certificate_error_callback_t) -> CefAllowCertificateErrorCallback {
    CefAllowCertificateErrorCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_allow_certificate_error_callback_t> for Option<CefAllowCertificateErrorCallback> {
  fn to_c(rust_object: Option<CefAllowCertificateErrorCallback>) -> *mut cef_allow_certificate_error_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_allow_certificate_error_callback_t) -> Option<CefAllowCertificateErrorCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefAllowCertificateErrorCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Implement this structure to handle events related to browser requests. The
// functions of this structure will be called on the thread indicated.
//
#[repr(C)]
pub struct _cef_request_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called on the UI thread before browser navigation. Return true (1) to
  // cancel the navigation or false (0) to allow the navigation to proceed. The
  // |request| object cannot be modified in this callback.
  // cef_load_handler_t::OnLoadingStateChange will be called twice in all cases.
  // If the navigation is allowed cef_load_handler_t::OnLoadStart and
  // cef_load_handler_t::OnLoadEnd will be called. If the navigation is canceled
  // cef_load_handler_t::OnLoadError will be called with an |errorCode| value of
  // ERR_ABORTED.
  //
  pub on_before_browse: Option<extern "C" fn(this: *mut cef_request_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      request: *mut interfaces::cef_request_t,
      is_redirect: libc::c_int) -> libc::c_int>,

  //
  // Called on the IO thread before a resource request is loaded. The |request|
  // object may be modified. To cancel the request return true (1) otherwise
  // return false (0).
  //
  pub on_before_resource_load: Option<extern "C" fn(
      this: *mut cef_request_handler_t, browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      request: *mut interfaces::cef_request_t) -> libc::c_int>,

  //
  // Called on the IO thread before a resource is loaded. To allow the resource
  // to load normally return NULL. To specify a handler for the resource return
  // a cef_resource_handler_t object. The |request| object should not be
  // modified in this callback.
  //
  pub get_resource_handler: Option<extern "C" fn(
      this: *mut cef_request_handler_t, browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      request: *mut interfaces::cef_request_t) -> *mut interfaces::cef_resource_handler_t>,

  //
  // Called on the IO thread when a resource load is redirected. The |old_url|
  // parameter will contain the old URL. The |new_url| parameter will contain
  // the new URL and can be changed if desired.
  //
  pub on_resource_redirect: Option<extern "C" fn(
      this: *mut cef_request_handler_t, browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t, old_url: *const types::cef_string_t,
      new_url: *mut types::cef_string_t) -> ()>,

  //
  // Called on the IO thread when the browser needs credentials from the user.
  // |isProxy| indicates whether the host is a proxy server. |host| contains the
  // hostname and |port| contains the port number. Return true (1) to continue
  // the request and call cef_auth_callback_t::cont() when the authentication
  // information is available. Return false (0) to cancel the request.
  //
  pub get_auth_credentials: Option<extern "C" fn(
      this: *mut cef_request_handler_t, browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t, isProxy: libc::c_int,
      host: *const types::cef_string_t, port: libc::c_int,
      realm: *const types::cef_string_t, scheme: *const types::cef_string_t,
      callback: *mut interfaces::cef_auth_callback_t) -> libc::c_int>,

  //
  // Called on the IO thread when JavaScript requests a specific storage quota
  // size via the webkitStorageInfo.requestQuota function. |origin_url| is the
  // origin of the page making the request. |new_size| is the requested quota
  // size in bytes. Return true (1) and call cef_quota_callback_t::cont() either
  // in this function or at a later time to grant or deny the request. Return
  // false (0) to cancel the request.
  //
  pub on_quota_request: Option<extern "C" fn(this: *mut cef_request_handler_t,
      browser: *mut interfaces::cef_browser_t,
      origin_url: *const types::cef_string_t, new_size: i64,
      callback: *mut interfaces::cef_quota_callback_t) -> libc::c_int>,

  //
  // Called on the UI thread to handle requests for URLs with an unknown
  // protocol component. Set |allow_os_execution| to true (1) to attempt
  // execution via the registered OS protocol handler, if any. SECURITY WARNING:
  // YOU SHOULD USE THIS METHOD TO ENFORCE RESTRICTIONS BASED ON SCHEME, HOST OR
  // OTHER URL ANALYSIS BEFORE ALLOWING OS EXECUTION.
  //
  pub on_protocol_execution: Option<extern "C" fn(
      this: *mut cef_request_handler_t, browser: *mut interfaces::cef_browser_t,
      url: *const types::cef_string_t,
      allow_os_execution: *mut libc::c_int) -> ()>,

  //
  // Called on the UI thread to handle requests for URLs with an invalid SSL
  // certificate. Return true (1) and call
  // cef_allow_certificate_error_callback_t:: cont() either in this function or
  // at a later time to continue or cancel the request. Return false (0) to
  // cancel the request immediately. If |callback| is NULL the error cannot be
  // recovered from and the request will be canceled automatically. If
  // CefSettings.ignore_certificate_errors is set all invalid certificates will
  // be accepted without calling this function.
  //
  pub on_certificate_error: Option<extern "C" fn(
      this: *mut cef_request_handler_t, cert_error: types::cef_errorcode_t,
      request_url: *const types::cef_string_t,
      callback: *mut interfaces::cef_allow_certificate_error_callback_t) -> libc::c_int>,

  //
  // Called on the browser process IO thread before a plugin is loaded. Return
  // true (1) to block loading of the plugin.
  //
  pub on_before_plugin_load: Option<extern "C" fn(
      this: *mut cef_request_handler_t, browser: *mut interfaces::cef_browser_t,
      url: *const types::cef_string_t, policy_url: *const types::cef_string_t,
      info: *mut interfaces::cef_web_plugin_info_t) -> libc::c_int>,

  //
  // Called on the browser process UI thread when a plugin has crashed.
  // |plugin_path| is the path of the plugin that crashed.
  //
  pub on_plugin_crashed: Option<extern "C" fn(this: *mut cef_request_handler_t,
      browser: *mut interfaces::cef_browser_t,
      plugin_path: *const types::cef_string_t) -> ()>,

  //
  // Called on the browser process UI thread when the render process terminates
  // unexpectedly. |status| indicates how the process terminated.
  //
  pub on_render_process_terminated: Option<extern "C" fn(
      this: *mut cef_request_handler_t, browser: *mut interfaces::cef_browser_t,
      status: types::cef_termination_status_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_request_handler_t = _cef_request_handler_t;


//
// Implement this structure to handle events related to browser requests. The
// functions of this structure will be called on the thread indicated.
//
pub struct CefRequestHandler {
  c_object: *mut cef_request_handler_t,
}

impl Clone for CefRequestHandler {
  fn clone(&self) -> CefRequestHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefRequestHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefRequestHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefRequestHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_request_handler_t) -> CefRequestHandler {
    CefRequestHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_request_handler_t) -> CefRequestHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefRequestHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_request_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_request_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Called on the UI thread before browser navigation. Return true (1) to
  // cancel the navigation or false (0) to allow the navigation to proceed. The
  // |request| object cannot be modified in this callback.
  // cef_load_handler_t::OnLoadingStateChange will be called twice in all cases.
  // If the navigation is allowed cef_load_handler_t::OnLoadStart and
  // cef_load_handler_t::OnLoadEnd will be called. If the navigation is canceled
  // cef_load_handler_t::OnLoadError will be called with an |errorCode| value of
  // ERR_ABORTED.
  //
  pub fn on_before_browse(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, request: interfaces::CefRequest,
      is_redirect: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_browse.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(request),
          CefWrap::to_c(is_redirect)))
    }
  }

  //
  // Called on the IO thread before a resource request is loaded. The |request|
  // object may be modified. To cancel the request return true (1) otherwise
  // return false (0).
  //
  pub fn on_before_resource_load(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame,
      request: interfaces::CefRequest) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_resource_load.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(request)))
    }
  }

  //
  // Called on the IO thread before a resource is loaded. To allow the resource
  // to load normally return NULL. To specify a handler for the resource return
  // a cef_resource_handler_t object. The |request| object should not be
  // modified in this callback.
  //
  pub fn get_resource_handler(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame,
      request: interfaces::CefRequest) -> interfaces::CefResourceHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_resource_handler.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(request)))
    }
  }

  //
  // Called on the IO thread when a resource load is redirected. The |old_url|
  // parameter will contain the old URL. The |new_url| parameter will contain
  // the new URL and can be changed if desired.
  //
  pub fn on_resource_redirect(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, old_url: &[u16],
      new_url: *mut types::cef_string_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_resource_redirect.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(old_url),
          CefWrap::to_c(new_url)))
    }
  }

  //
  // Called on the IO thread when the browser needs credentials from the user.
  // |isProxy| indicates whether the host is a proxy server. |host| contains the
  // hostname and |port| contains the port number. Return true (1) to continue
  // the request and call cef_auth_callback_t::cont() when the authentication
  // information is available. Return false (0) to cancel the request.
  //
  pub fn get_auth_credentials(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, isProxy: libc::c_int, host: &[u16],
      port: libc::c_int, realm: &[u16], scheme: &[u16],
      callback: interfaces::CefAuthCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_auth_credentials.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(isProxy),
          CefWrap::to_c(host),
          CefWrap::to_c(port),
          CefWrap::to_c(realm),
          CefWrap::to_c(scheme),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Called on the IO thread when JavaScript requests a specific storage quota
  // size via the webkitStorageInfo.requestQuota function. |origin_url| is the
  // origin of the page making the request. |new_size| is the requested quota
  // size in bytes. Return true (1) and call cef_quota_callback_t::cont() either
  // in this function or at a later time to grant or deny the request. Return
  // false (0) to cancel the request.
  //
  pub fn on_quota_request(&self, browser: interfaces::CefBrowser,
      origin_url: &[u16], new_size: i64,
      callback: interfaces::CefQuotaCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_quota_request.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(origin_url),
          CefWrap::to_c(new_size),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Called on the UI thread to handle requests for URLs with an unknown
  // protocol component. Set |allow_os_execution| to true (1) to attempt
  // execution via the registered OS protocol handler, if any. SECURITY WARNING:
  // YOU SHOULD USE THIS METHOD TO ENFORCE RESTRICTIONS BASED ON SCHEME, HOST OR
  // OTHER URL ANALYSIS BEFORE ALLOWING OS EXECUTION.
  //
  pub fn on_protocol_execution(&self, browser: interfaces::CefBrowser,
      url: &[u16], allow_os_execution: &mut libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_protocol_execution.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(url),
          CefWrap::to_c(allow_os_execution)))
    }
  }

  //
  // Called on the UI thread to handle requests for URLs with an invalid SSL
  // certificate. Return true (1) and call
  // cef_allow_certificate_error_callback_t:: cont() either in this function or
  // at a later time to continue or cancel the request. Return false (0) to
  // cancel the request immediately. If |callback| is NULL the error cannot be
  // recovered from and the request will be canceled automatically. If
  // CefSettings.ignore_certificate_errors is set all invalid certificates will
  // be accepted without calling this function.
  //
  pub fn on_certificate_error(&self, cert_error: types::cef_errorcode_t,
      request_url: &[u16],
      callback: interfaces::CefAllowCertificateErrorCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_certificate_error.unwrap())(
          self.c_object,
          CefWrap::to_c(cert_error),
          CefWrap::to_c(request_url),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Called on the browser process IO thread before a plugin is loaded. Return
  // true (1) to block loading of the plugin.
  //
  pub fn on_before_plugin_load(&self, browser: interfaces::CefBrowser,
      url: &[u16], policy_url: &[u16],
      info: interfaces::CefWebPluginInfo) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_plugin_load.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(url),
          CefWrap::to_c(policy_url),
          CefWrap::to_c(info)))
    }
  }

  //
  // Called on the browser process UI thread when a plugin has crashed.
  // |plugin_path| is the path of the plugin that crashed.
  //
  pub fn on_plugin_crashed(&self, browser: interfaces::CefBrowser,
      plugin_path: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_plugin_crashed.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(plugin_path)))
    }
  }

  //
  // Called on the browser process UI thread when the render process terminates
  // unexpectedly. |status| indicates how the process terminated.
  //
  pub fn on_render_process_terminated(&self, browser: interfaces::CefBrowser,
      status: types::cef_termination_status_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_render_process_terminated.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(status)))
    }
  }
} 

impl CefWrap<*mut cef_request_handler_t> for CefRequestHandler {
  fn to_c(rust_object: CefRequestHandler) -> *mut cef_request_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_request_handler_t) -> CefRequestHandler {
    CefRequestHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_request_handler_t> for Option<CefRequestHandler> {
  fn to_c(rust_object: Option<CefRequestHandler>) -> *mut cef_request_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_request_handler_t) -> Option<CefRequestHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefRequestHandler::from_c_object_addref(c_object))
    }
  }
}
