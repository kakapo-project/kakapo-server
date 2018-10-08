-- keep track of versions

CREATE TABLE version (
    version_id            SERIAL PRIMARY KEY,
    version_update        VARCHAR NOT NULL UNIQUE,
    update_at             TIMESTAMP NOT NULL DEFAULT NOW()
);