use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tide::{Middleware, Next, Request, Result};
use std::os::raw::c_int;
use std::ffi::CString;
use std::os::raw::c_char;

/// Trace utility for incoming requests and responses.
///
/// # Examples
///
/// ```
///     let mut app = tide::new();
///     app.middleware(tide_trace::USDTMiddleware::new(0));
/// ```
#[derive(Debug, Clone)]
pub struct USDTMiddleware {
    reqid: Arc<AtomicUsize>,
}
impl USDTMiddleware {
    /// Create a new instance of `USDTMiddleware`.
    pub fn new(start: usize) -> Self {
        Self {
            reqid: Arc::new(AtomicUsize::new(start)),
        }
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for USDTMiddleware {
    fn handle<'a>(
        &'a self,
        req: Request<State>,
        next: Next<'a, State>,
    ) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
        Box::pin(async move {
            let count = self.reqid.fetch_add(1, Ordering::Relaxed);
            let path = CString::new(req.url().path().to_owned()).expect("CString::new path failed");
            let method = CString::new(req.method().to_string()).expect("CString::new method failed");
            let c_tid = CString::new(count.to_string()).expect("CString::new method failed");

            unsafe {
                startroute(method.as_ptr(), path.as_ptr(), c_tid.as_ptr());
            }
  
            let res = next.run(req).await?;
            let status = res.status() as i32;
            // let c_status = i32::from(status);
            // let headers = res.
            unsafe {
                endroute(method.as_ptr(), path.as_ptr(),  c_tid.as_ptr(), status, c_tid.as_ptr());
            }
            Ok(res)
        })
    }
}

extern "C" {
    fn startroute(method: *const c_char, path: *const c_char, id: *const c_char);
    fn endroute(method: *const c_char, path: *const c_char, id: *const c_char, status: c_int, headers: *const c_char);
    fn probe(tag: *const c_char, data: *const c_char);
}

/// Trace utility for incoming requests and responses.
///
/// # Examples
///
/// ```
/// let opt: Option<String> = Some("identifier".to_string());
/// tide_trace::fire("data to log".to_string(), opt);
/// ```
    pub fn fire(data : String, tag: Option<String>) {
        let c_tag : CString;

        if let Some(tag) = tag {
            c_tag = CString::new(tag).expect("CString::new tag failed");
        } else {
            c_tag = CString::new("default").expect("CString::new tag failed");
        }
        
        let c_data = CString::new(data).expect("CString::new data failed");
        unsafe {
            probe(c_tag.as_ptr(), c_data.as_ptr());
        }
    }
