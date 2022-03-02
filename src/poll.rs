use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Poll {
    pub id: Ulid,
    pub title: String,
    pub multi: bool,
    pub options: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PollOption {
    pub poll_option_id: Ulid,
    pub option: String,
    pub votes: i32,
    pub related_poll_id: Ulid,
}

// poll_option_id          BLOB PRIMARY KEY    NOT NULL,
// poll_id     BLOB                NOT NULL,
// option      TEXT                NOT NULL,
// votes       INT                 NOT NULL    DEFAULT 0,
// constraint fk_polls foreign key(poll_id) references polls(id)

#[derive(Deserialize, Serialize)]
pub struct CreatePoll {
    pub title: String,
    pub options: Vec<String>,
    pub multi: bool,
}
