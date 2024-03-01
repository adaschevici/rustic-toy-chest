use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| {
        match err {
            ClientError::ServerError { status, message: _ } => match status {
                404 => (
                    view! { cx,
                        title { "Page not found" }
                    },
                    view! { cx,
                        p { "Page now found" }
                    },
                ),
                // 4xx is a client error
                _ if (400..500).contains(&status) => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "bad request" }
                    },
                ),
                // 5xx is a server error
                _ => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "Sorry, internal server error." }
                    },
                ),
            },
            ClientError::Panic(_) => (
                view! { cx,
                    title { "Critical error" }
                },
                view! { cx,
                    p { "Sorry, something went wrong" }
                },
            ),
            ClientError::FetchError(_) => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { "Network error, ensur you have stable internet connection)" }
                },
            ),
            _ => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { (format!("Internal server error: '{}'.", err)) }
                },
            ),
        }
    })
}
