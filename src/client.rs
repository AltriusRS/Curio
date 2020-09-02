use crate::structs::{Connection, Response, Request};
use crate::types::Result as CurioResult;

pub fn get(connection: &Connection, req: &Request) -> CurioResult<Response> {
    return Err(crate::types::err_from_code(0));
}