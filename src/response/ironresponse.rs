use HttpResponse = http::server::response::ResponseWriter;
use http::server::request::Request;
use http::headers::response::HeaderCollection;
use http::status::Status;

use std::io::IoResult;
use std::mem;

use super::Response;

pub struct IronResponse<'a, 'b> {
    writer: &'a mut HttpResponse<'b>
}

impl <'a, 'b> Writer for IronResponse<'a, 'b> {
    fn write(&mut self, content: &[u8]) -> IoResult<()> {
        self.writer.write(content)
    }
}

impl<'a, 'b> Response<'a, 'b> for IronResponse<'a, 'b> {
    #[inline]
    fn headers_mut<'a>(&'a mut self) -> &'a mut Box<HeaderCollection> { &mut self.writer.headers }

    #[inline]
    fn status_mut<'a>(&'a mut self) -> &'a mut Status { &mut self.writer.status }

    #[inline]
    fn request<'a>(&'a self) -> &'a Request { self.writer.request }

    #[inline]
    fn headers<'a>(&'a self) -> &'a HeaderCollection { &*self.writer.headers }

    #[inline]
    fn status<'a>(&'a self) -> &'a Status { &self.writer.status }

    #[inline]
    fn from_http(writer: &mut HttpResponse) -> IronResponse<'a, 'b> {
        unsafe {
            mem::transmute(
                IronResponse {
                    writer: writer
                }
            )
        }
    }
}

