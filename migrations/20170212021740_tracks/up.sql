CREATE TABLE tracks (
      track_id SERIAL PRIMARY KEY,
      path VARCHAR NOT NULL,
      title TEXT NOT NULL,
      album TEXT NOT NULL,
      artist TEXT NOT NULL
)
