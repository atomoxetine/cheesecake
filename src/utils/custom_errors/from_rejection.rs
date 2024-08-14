use axum::extract::rejection::*;

use super::app_error::AppError;

macro_rules! from_rejection {
    ($from:ty, $rejection:ty ) => {
        impl From<$from> for $rejection {
            fn from(value: $from) -> Self {
                Self::new(value.body_text(), value.status(), None, false)
            }
        }
    };
}

// Insert here all rejections that might be used in your application to automatically
// convert them to AppError. This way you can use them inside your controllers with
// the wrappers `WithApiRejection` and `WithViewRejection` seaminglessly.
from_rejection!(FormRejection, AppError);
from_rejection!(HostRejection, AppError);
from_rejection!(JsonRejection, AppError);
from_rejection!(PathRejection, AppError);
from_rejection!(BytesRejection, AppError);
from_rejection!(QueryRejection, AppError);
from_rejection!(StringRejection, AppError);
from_rejection!(RawFormRejection, AppError);
from_rejection!(ExtensionRejection, AppError);
from_rejection!(FailedToBufferBody, AppError);
from_rejection!(NestedPathRejection, AppError);
from_rejection!(MatchedPathRejection, AppError);
from_rejection!(RawPathParamsRejection, AppError);
