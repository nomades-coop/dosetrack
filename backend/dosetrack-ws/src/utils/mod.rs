use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

#[derive(Debug)]
pub struct GenericError {
    status: Status,
    json: String,
}

impl GenericError {
    pub fn new(status: Status, data: &str) -> GenericError {
        GenericError {
            status,
            json: String::from(data),
        }
    }
}

impl<'r> Responder<'r, 'static> for GenericError {
    fn respond_to(self, request: &Request) -> rocket::response::Result<'static> {
        Response::build()
            .header(ContentType::JSON)
            .status(self.status)
            .sized_body(self.json.len(), Cursor::new(self.json))
            .ok()
    }
}

// Response::build_from(self.json.respond_to(&request).unwrap())
// .header(ContentType::JSON)
// .status(self.status)
// .ok()

// impl<'r> Responder<'r, 'r> for GenericError {
//     fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'r> {
//         let mut response = Response::build()
//             .header(ContentType::HTML)
//             .status(rocket::http::Status::from_code(self.status).unwrap())
//             .finalize();

//         let json = format!(
//             r###"{{"status": {0}, "detail": "{1}"}}"###,
//             self.status, &*self.details
//         );
//         response.set_sized_body(json.len(), io::Cursor::new(json));
//         Ok(response)
//     }
// }
