use limine::request::FramebufferRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

use super::{BootInfo, FramebufferInfo};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

/// Initialize the Limine boot layer.
pub fn init() -> BootInfo {
    let framebuffer = FRAMEBUFFER_REQUEST.response().and_then(|response| {
        response
            .framebuffers()
            .first()
            .map(|framebuffer| FramebufferInfo {
                address: framebuffer.address() as *mut u8,
                pitch: framebuffer.pitch as usize,
                width: framebuffer.width as usize,
                height: framebuffer.height as usize,
            })
    });

    BootInfo { framebuffer }
}
