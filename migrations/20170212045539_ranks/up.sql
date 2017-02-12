CREATE TABLE ranks (
    song_id INTEGER REFERENCES tracks (id),
    user_id VARCHAR REFERENCES users (user_id),
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY(song_id, user_id, timestamp)
)
