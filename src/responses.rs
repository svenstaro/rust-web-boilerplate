use std::io::Cursor;
use rocket_contrib::Value;
use rocket::response::{Response, Responder};
use rocket::http::{Status, ContentType};

#[derive(Debug)]
pub struct APIResponse {
    message: Option<String>,
    data: Option<Value>,
    status: Status,
}

impl APIResponse {
    /// Change the message of the `Response`.
    pub fn message(mut self, message: &str) -> APIResponse {
        self.message = Some(message.to_string());
        self
    }

    /// Change the data to the `Response`.
    pub fn data(mut self, data: Value) -> APIResponse {
        self.data = Some(data);
        self
    }
}

impl<'r> Responder<'r> for APIResponse {
    fn respond(self) -> Result<Response<'r>, Status> {
        let body = json!({
            "message": self.message,
            "data": self.data,
        });

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn ok() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::Ok,
    }
}

pub fn created() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::Created,
    }
}

pub fn accepted() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::Accepted,
    }
}

pub fn no_content() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::NoContent,
    }
}


pub fn bad_request() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::NoContent,
    }
}

pub fn unauthorized() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::Unauthorized,
    }
}

pub fn forbidden() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::Forbidden,
    }
}

pub fn not_found() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::NotFound,
    }
}

pub fn method_not_allowed() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::MethodNotAllowed,
    }
}

pub fn conflict() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::Conflict,
    }
}

pub fn unprocessable_entity() -> APIResponse {
    APIResponse {
        message: None,
        data: None,
        status: Status::UnprocessableEntity,
    }
}
