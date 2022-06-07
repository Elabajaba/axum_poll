CREATE TABLE IF NOT EXISTS polls
(
    poll_id     BLOB PRIMARY KEY    NOT NULL,
    title       TEXT                NOT NULL,
    multi       BOOLEAN             NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS poll_options
(
    poll_option_id          BLOB PRIMARY KEY    NOT NULL,
    poll_id     BLOB                NOT NULL,
    option      TEXT                NOT NULL,
    votes       INT                 NOT NULL    DEFAULT 0,
    constraint fk_polls foreign key(poll_id) references polls(poll_id)
);