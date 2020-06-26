use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicI32, Ordering};
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
///     app.middleware(tide_trace::USDTMiddleware::new());
/// ```
#[derive(Debug, Clone)]
pub struct USDTMiddleware {
    reqid: Arc<AtomicI32>,
}
impl USDTMiddleware {
    /// Create a new instance of `USDTMiddleware`.
    pub fn new() -> Self {
        Self {
            reqid: Arc::new(<AtomicI32>::new(1)),
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
            let count = self.reqid.fetch_add(1, Ordering::SeqCst);
            let path = CString::new(req.url().path().to_owned()).expect("CString::new path failed");
            let method = CString::new(req.method().to_string()).expect("CString::new method failed");
            let req_headers_coll = req.iter().map(|h| format!("{:?};", h)) 
            .collect::<Vec<String>>()
            .join("; ");
            let req_headers = CString::new(req_headers_coll).expect("CString::new header failed");
            unsafe {
                startroute(method.as_ptr(), path.as_ptr(), count, req_headers.as_ptr());
            }
  
            let res = next.run(req).await?;
            let status = res.status() as i32;
            let res_headers_coll = res.iter().map(|h| format!("{:?};", h)) 
            .collect::<Vec<String>>()
            .join("; ");
            let res_headers = CString::new(res_headers_coll).expect("CString::new header failed");
            unsafe {
                endroute(method.as_ptr(), path.as_ptr(),  count, status, res_headers.as_ptr());
            }
            self.reqid.load(Ordering::SeqCst);
            Ok(res)
        })
    }
}

extern "C" {
    fn startroute(method: *const c_char, path: *const c_char, reqid: c_int, headers: *const c_char);
    fn endroute(method: *const c_char, path: *const c_char, reqid: c_int, status: c_int, headers: *const c_char);
    fn fire(tag: *const c_char, data: *const c_char);
}

/// Trace utility for incoming requests and responses.
///
/// # Examples
///
/// ```
/// let opt: Option<String> = Some("identifier".to_string());
/// tide_trace::probe("data to log".to_string(), opt);
/// ```
    pub fn probe(data : String, tag: Option<String>) {
        let c_tag : CString;

        if let Some(tag) = tag {
            c_tag = CString::new(tag).expect("CString::new tag failed");
        } else {
            c_tag = CString::new("default").expect("CString::new tag failed");
        }
        
        let c_data = CString::new(data).expect("CString::new data failed");
        unsafe {
            fire(c_tag.as_ptr(), c_data.as_ptr());
        }
    }
