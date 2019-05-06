use rocket::Request;
use rocket::http::Status;
use rocket::response::{Responder, Response};

use crate::errors::Error;

pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<'r, A, B> Responder<'r> for Either<A, B> where A: Responder<'r>, B: Responder<'r> {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        match self {
            Either::A(responder) => responder.respond_to(request),
            Either::B(responder) => responder.respond_to(request),
        }
    }
}
