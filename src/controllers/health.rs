use rocket::State;
use rocket_contrib::json::Json;
use crate::models::response::{Message, Response};
use crate::PortConfig;

#[get("/")]
pub fn index(state: State<PortConfig>) -> Json<Response<Message>> {
    let mut response = Response::ok();
    response.msg(format!("Servidor executando na porta {}", state.0));
    Json(response)
}