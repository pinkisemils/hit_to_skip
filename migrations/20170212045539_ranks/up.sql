CREATE TABLE ranks (
    track_id INTEGER REFERENCES tracks (track_id),
    user_id VARCHAR REFERENCES users (user_id),
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY(track_id, user_id, timestamp)
)
