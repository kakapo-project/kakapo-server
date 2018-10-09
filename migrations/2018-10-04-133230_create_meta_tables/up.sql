-- Initialize the meta tables

CREATE TABLE user_account (
    user_account_id         BIGSERIAL PRIMARY KEY,
    username                VARCHAR NOT NULL UNIQUE,
    password                VARCHAR NOT NULL,
    email                   VARCHAR NOT NULL UNIQUE
);

CREATE TABLE scope (
    scope_id                BIGSERIAL PRIMARY KEY,
    name                    VARCHAR NOT NULL UNIQUE,
    description             VARCHAR NOT NULL DEFAULT '',
    scope_info              JSON NOT NULL DEFAULT '{}'
);

-- TODO: entity should be unique across `table_schema` and `query` and `script`
CREATE TABLE entity (
    entity_id               BIGSERIAL PRIMARY KEY,
    scope_id                BIGINT REFERENCES scope NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by              BIGINT REFERENCES user_account NOT NULL
);

CREATE TABLE table_schema (
    table_schema_id         BIGSERIAL PRIMARY KEY,
    entity_id               BIGINT REFERENCES entity REFERENCES entity NOT NULL UNIQUE,
    name                    VARCHAR NOT NULL UNIQUE
);

CREATE TABLE table_schema_history (
    table_schema_history_id BIGSERIAL PRIMARY KEY,
    table_schema_id         BIGINT REFERENCES table_schema NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    modification            JSON NOT NULL DEFAULT '{}',
    modified_at             TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_by             BIGINT REFERENCES user_account NOT NULL
);

CREATE TABLE query (
    query_id                BIGSERIAL PRIMARY KEY,
    entity_id               BIGINT REFERENCES entity REFERENCES entity NOT NULL UNIQUE,
    name                    VARCHAR NOT NULL UNIQUE
);

CREATE TABLE query_history (
    query_history_id        BIGSERIAL PRIMARY KEY,
    query_id                BIGINT REFERENCES query NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    statement               VARCHAR NOT NULL,
    query_info              JSON NOT NULL DEFAULT '{}',
    modified_at             TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_by             BIGINT REFERENCES user_account NOT NULL
);

CREATE TABLE script (
    script_id               BIGSERIAL PRIMARY KEY,
    entity_id               BIGINT REFERENCES entity REFERENCES entity NOT NULL UNIQUE,
    name                    VARCHAR NOT NULL UNIQUE
);

CREATE TABLE script_history (
    script_history_id       BIGSERIAL PRIMARY KEY,
    script_id               BIGINT REFERENCES script NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    script_language         VARCHAR NOT NULL,
    script_text             VARCHAR NOT NULL,
    script_info             JSON NOT NULL DEFAULT '{}',
    modified_at             TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_by             BIGINT REFERENCES user_account NOT NULL
);

CREATE TABLE tag (
    tag_id                  BIGSERIAL PRIMARY KEY,
    name                    VARCHAR NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    tag_info                JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE entity_tag (
    entity_tag_id           BIGSERIAL PRIMARY KEY,
    entity_id               BIGINT REFERENCES entity NOT NULL,
    tag_id                  BIGINT REFERENCES tag NOT NULL
);

CREATE TABLE role (
    role_id                 BIGSERIAL PRIMARY KEY,
    name                    VARCHAR NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    role_info               JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE user_account_role (
    user_account_role_id   BIGSERIAL PRIMARY KEY,
    user_account_id        BIGINT REFERENCES user_account NOT NULL,
    role_id                BIGINT REFERENCES role NOT NULL
);

CREATE TABLE permission (
    permission_id          BIGSERIAL PRIMARY KEY,
    name                   VARCHAR NOT NULL,
    description            VARCHAR NOT NULL DEFAULT '',
    permission_info        JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE role_permission (
    role_permission_id     BIGSERIAL PRIMARY KEY,
    role_id                BIGINT REFERENCES role NOT NULL,
    permission_id          BIGINT REFERENCES permission NOT NULL
);

CREATE TABLE table_schema_transaction (
    transaction_id         BIGSERIAL PRIMARY KEY,
    version                VARCHAR NOT NULL DEFAULT '0.1.0',
    action_data            JSON NOT NULL,
    table_schema_id        BIGINT REFERENCES table_schema NOT NULL,
    made_at                TIMESTAMP NOT NULL DEFAULT NOW(),
    made_by                BIGINT REFERENCES user_account NOT NULL
);

INSERT INTO version (version_update) VALUES ('0.1.0');
