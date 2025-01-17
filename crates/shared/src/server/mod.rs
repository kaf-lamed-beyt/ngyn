pub mod body;
pub mod context;
pub mod response;
pub mod transformer;
pub mod uri;

pub use self::response::{JsonResponse, JsonResult};
pub use body::ToBytes;
pub use bytes::Bytes;
pub use context::NgynContext;
pub use http::Method;
use http_body_util::Full;
pub use transformer::{Body, Param, Query, Transducer, Transformer};

pub type NgynRequest = http::Request<Vec<u8>>;
pub type NgynResponse = http::Response<Full<Bytes>>;

pub(crate) type Routes = Vec<(String, Method, Box<crate::core::Handler>)>;
pub(crate) type Middlewares = Vec<Box<dyn crate::traits::NgynMiddleware>>;
