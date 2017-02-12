CREATE TABLE tracks (
      track_id SERIAL PRIMARY KEY,
      path VARCHAR NOT NULL,
      title VARCHAR NOT NULL,
      album VARCHAR NOT NULL,
      artist VARCHAR NOT NULL
)
