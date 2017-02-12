-- no songs are bad, there are just less upvoted songs
CREATE TABLE ranks (
    song_id INTEGER REFERENCES tracks (id),
    user_id VARCHAR REFERENCES users (id),
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY_KEY(song_id, user_id, timestamp),
)
