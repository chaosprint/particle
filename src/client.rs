use ambient_api::prelude::*;

#[main]
pub fn main() {
    // let mut cursor_lock = input::CursorLockGuard::new(true);
    ambient_api::messages::Frame::subscribe(move |_| {
        let (delta, input) = input::get_delta();
        // if !cursor_lock.auto_unlock_on_escape(&input) {
        //     return;
        // }

        if delta.keys.contains(&KeyCode::Space) {
            println!("shoot");
            messages::Input::new(true).send_server_reliable();
        }
    });
}
