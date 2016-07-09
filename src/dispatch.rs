use ::error::{TResult, TError};
use ::messaging;
use ::util::json;
use ::util::json::Value;
use ::models::user;

/// process a message from the messaging system. this is the main communication
/// heart of turtl core.
pub fn process(msg: &String) -> TResult<()> {
    let data: Value = try_t!(json::parse(msg));

    // grab the command from the data
    let cmd: String = try_t!(json::get(&["0"], &data));

    match cmd.as_ref() {
        "user:login" => {
            let username = try_t!(json::get(&["1", "username"], &data));
            let password = try_t!(json::get(&["1", "password"], &data));
            user::login(username, password)
        },
        "ping" => {
            info!("ping!");
            return messaging::send(&"{\"e\":\"pong\"}".to_owned())
        }
        "shutdown" => return Err(TError::Shutdown),
        _ => Err(TError::Msg(format!("bad command: {}", cmd))),
    }
}

/// our main dispatch loop. really, just calls into messaging::bind and hands it
/// our process function
pub fn main() {
    match messaging::bind(&process) {
        Ok(..) => (),
        Err(e) => panic!("dispatch: error starting messaging system: {}", e),
    }
}

