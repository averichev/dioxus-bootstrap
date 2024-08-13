use dioxus::prelude::*;
use uuid::Uuid;

pub fn use_uid_generator() -> Memo<Uuid> {
    use_memo(Uuid::new_v4)
}