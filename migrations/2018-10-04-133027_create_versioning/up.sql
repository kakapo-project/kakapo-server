-- keep track of versions

CREATE TABLE "version" (
    "version_id"            BIGSERIAL PRIMARY KEY,
    "version_update"        VARCHAR NOT NULL UNIQUE,
    "updated_at"            TIMESTAMP NOT NULL DEFAULT NOW()
);