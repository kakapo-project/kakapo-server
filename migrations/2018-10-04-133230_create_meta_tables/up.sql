-- Initialize the meta tables

CREATE TABLE "domain" (
    "domain_id"               BIGSERIAL PRIMARY KEY,
    "name"                    VARCHAR NOT NULL UNIQUE,
    "type"                    VARCHAR NOT NULL,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "domain_info"             JSON NOT NULL DEFAULT '{}',
    "created_at"              TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE "user" (
    "user_id"                 BIGSERIAL PRIMARY KEY,
    "username"                VARCHAR NOT NULL UNIQUE,
    "email"                   VARCHAR NOT NULL UNIQUE,
    "password"                VARCHAR NOT NULL, --TODO: store last_updated field
    "display_name"            VARCHAR NOT NULL,
    "user_info"               JSON NOT NULL DEFAULT '{}',
    "joined_at"               TIMESTAMP NOT NULL DEFAULT NOW(),
    CHECK ("email" LIKE '%_@__%.__%'), -- has false positives but no false negatives
    CHECK ("username" NOT LIKE '%@%') -- username can't have @ sign otherwise it might conflict with email
);

CREATE TABLE "invitation" (
    "invitation_id"           BIGSERIAL PRIMARY KEY,
    "email"                   VARCHAR NOT NULL UNIQUE,
    "token"                   VARCHAR NOT NULL, -- TODO: should be unique
    "token_info"              JSON NOT NULL DEFAULT '{}',
    "sent_at"                 TIMESTAMP NOT NULL DEFAULT NOW(),
    "expires_at"              TIMESTAMP NOT NULL DEFAULT (NOW() + INTERVAL '1 DAY'),
    CHECK ("email" LIKE '%_@__%.__%') -- has false positives but no false negatives
);

CREATE TABLE "session" (
    "session_id"              BIGSERIAL PRIMARY KEY,
    "token"                   VARCHAR NOT NULL, -- TODO: should be unique
    "user_id"                 BIGINT REFERENCES "user" NOT NULL,
    "created_at"              TIMESTAMP NOT NULL DEFAULT NOW(),
    "expires_at"              TIMESTAMP NOT NULL DEFAULT (NOW() + INTERVAL '1 DAY')
);

CREATE TABLE "scope" (
    "scope_id"                BIGSERIAL PRIMARY KEY,
    "name"                    VARCHAR NOT NULL UNIQUE,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "scope_info"              JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE "channel" (
    "channel_id"              BIGSERIAL PRIMARY KEY,
    "data"                    JSONB NOT NULL UNIQUE
);

CREATE TABLE "user_channel" ( -- This is the subscriptions
    "user_channel_id"         BIGSERIAL PRIMARY KEY,
    "user_id"                 BIGINT REFERENCES "user" ON DELETE CASCADE NOT NULL , --TODO: maybe this should be session
    "channel_id"              BIGINT REFERENCES "channel" ON DELETE CASCADE NOT NULL ,
    "subscribed_at"           TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE ("user_id", "channel_id")
);

CREATE TABLE "message" (
    "message_id"              BIGSERIAL PRIMARY KEY,
    "channel_id"              BIGINT REFERENCES "channel" ON DELETE CASCADE NOT NULL ,
    "data"                    JSONB NOT NULL,
    "sent_at"                 TIMESTAMP NOT NULL DEFAULT NOW()
);

-- TODO: entity should be unique across `table_schema` and `query` and `script`
CREATE TABLE "entity" (
    "entity_id"               BIGSERIAL PRIMARY KEY,
    "scope_id"                BIGINT REFERENCES "scope" NOT NULL,
    "created_at"              TIMESTAMP NOT NULL DEFAULT NOW(),
    "created_by"              BIGINT REFERENCES "user" NOT NULL
);

CREATE TABLE "entity_usage" (
    "entity_usage_id"         BIGSERIAL PRIMARY KEY,
    "entity_id"               BIGINT REFERENCES "entity" NOT NULL,
    "used_at"                 TIMESTAMP NOT NULL DEFAULT NOW(),
    "used_by"                 BIGINT REFERENCES "user" NOT NULL
);

CREATE TABLE "table_schema" (
    "table_schema_id"         BIGSERIAL PRIMARY KEY,
    "entity_id"               BIGINT REFERENCES "entity" NOT NULL,
    "name"                    VARCHAR NOT NULL,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "table_data"              JSON NOT NULL DEFAULT '{}',
    "is_deleted"              BOOLEAN NOT NULL DEFAULT FALSE,
    "modified_at"             TIMESTAMP NOT NULL DEFAULT NOW(),
    "modified_by"             BIGINT REFERENCES "user" NOT NULL
);

CREATE TABLE "query" (
    "query_id"                BIGSERIAL PRIMARY KEY,
    "entity_id"               BIGINT REFERENCES "entity" NOT NULL,
    "name"                    VARCHAR NOT NULL,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "statement"               VARCHAR NOT NULL,
    "query_info"              JSON NOT NULL DEFAULT '{}',
    "is_deleted"              BOOLEAN NOT NULL DEFAULT FALSE,
    "modified_at"             TIMESTAMP NOT NULL DEFAULT NOW(),
    "modified_by"             BIGINT REFERENCES "user" NOT NULL
);

CREATE TABLE "script" (
    "script_id"               BIGSERIAL PRIMARY KEY,
    "entity_id"               BIGINT REFERENCES "entity" NOT NULL,
    "name"                    VARCHAR NOT NULL,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "script_language"         VARCHAR NOT NULL,
    "script_text"             VARCHAR NOT NULL,
    "script_info"             JSON NOT NULL DEFAULT '{}',
    "is_deleted"              BOOLEAN NOT NULL DEFAULT FALSE,
    "modified_at"             TIMESTAMP NOT NULL DEFAULT NOW(),
    "modified_by"             BIGINT REFERENCES "user" NOT NULL
);

CREATE TABLE "view" (
    "view_id"                 BIGSERIAL PRIMARY KEY,
    "entity_id"               BIGINT REFERENCES "entity" NOT NULL,
    "name"                    VARCHAR NOT NULL,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "view_state"              JSON NOT NULL DEFAULT '{}',
    "view_info"               JSON NOT NULL DEFAULT '{}',
    "is_deleted"              BOOLEAN NOT NULL DEFAULT FALSE,
    "modified_at"             TIMESTAMP NOT NULL DEFAULT NOW(),
    "modified_by"             BIGINT REFERENCES "user" NOT NULL
);

CREATE TABLE "tag" (
    "tag_id"                  BIGSERIAL PRIMARY KEY,
    "name"                    VARCHAR NOT NULL UNIQUE,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "tag_info"                JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE "entity_tag" (
    "entity_tag_id"           BIGSERIAL PRIMARY KEY,
    "entity_id"               BIGINT REFERENCES "entity" ON DELETE CASCADE NOT NULL ,
    "tag_id"                  BIGINT REFERENCES "tag" ON DELETE CASCADE NOT NULL ,
    UNIQUE ("entity_id", "tag_id")
);

CREATE TABLE "role" (
    "role_id"                 BIGSERIAL PRIMARY KEY,
    "name"                    VARCHAR NOT NULL UNIQUE,
    "description"             VARCHAR NOT NULL DEFAULT '',
    "role_info"               JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE "user_role" (
    "user_role_id"            BIGSERIAL PRIMARY KEY,
    "user_id"                 BIGINT REFERENCES "user" ON DELETE CASCADE NOT NULL ,
    "role_id"                 BIGINT REFERENCES "role" ON DELETE CASCADE NOT NULL ,
    UNIQUE ("user_id", "role_id")
);

CREATE TABLE "permission" (
    "permission_id"           BIGSERIAL PRIMARY KEY,
    "data"                    JSONB NOT NULL UNIQUE
);

CREATE TABLE "role_permission" (
    "role_permission_id"      BIGSERIAL PRIMARY KEY,
    "role_id"                 BIGINT REFERENCES "role"ON DELETE CASCADE NOT NULL ,
    "permission_id"           BIGINT REFERENCES "permission" ON DELETE CASCADE NOT NULL ,
    UNIQUE ("role_id", "permission_id")
);

CREATE TABLE "table_schema_transaction" (
    "transaction_id"          BIGSERIAL PRIMARY KEY,
    "version"                 VARCHAR NOT NULL DEFAULT '0.1.0',
    "action_data"             JSON NOT NULL,
    "table_schema_id"         BIGINT REFERENCES "table_schema" NOT NULL,  --TODO: not sure if this is right, should be entity_id?
    "made_at"                 TIMESTAMP NOT NULL DEFAULT NOW(),
    "made_by"                 BIGINT REFERENCES "user" NOT NULL
);

INSERT INTO "version" ("version_update") VALUES ('0.1.0');
