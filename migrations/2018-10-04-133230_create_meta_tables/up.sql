-- Initialize the meta tables

CREATE TABLE user_account (
    user_account_id        SERIAL PRIMARY KEY,
    username               VARCHAR NOT NULL,
    password               VARCHAR NOT NULL,
    email                  VARCHAR NOT NULL
);

CREATE TABLE scope (
    scope_id               SERIAL PRIMARY KEY,
    name                   VARCHAR NOT NULL,
    description            VARCHAR NOT NULL DEFAULT '',
    scope_info             JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE entity (
    entity_id              SERIAL PRIMARY KEY,
    entity_type            VARCHAR CHECK (entity_type IN ('spread_sheet', 'query', 'script')),
    scope_id               INTEGER REFERENCES scope,
    created_at             TIMESTAMP NOT NULL DEFAULT NOW(),
    create_by              INTEGER REFERENCES user_account,
    UNIQUE (entity_id, entity_type)
);

CREATE TABLE spread_sheet (
    spread_sheet_id        SERIAL PRIMARY KEY,
    entity_id              INTEGER REFERENCES entity,
    entity_type            VARCHAR CHECK (entity_type = 'spread_sheet') DEFAULT 'spread_sheet',
    FOREIGN KEY (entity_id, entity_type) REFERENCES entity (entity_id, entity_type)
);

CREATE TABLE spread_sheet_history (
    spread_sheet_history_id SERIAL PRIMARY KEY,
    spread_sheet_id         INTEGER REFERENCES spread_sheet,
    name                    VARCHAR NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    spread_sheet_info       JSON NOT NULL DEFAULT '{}',
    modified_at             TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_by             INTEGER REFERENCES user_account
);

CREATE TABLE query (
    query_id                SERIAL PRIMARY KEY,
    entity_id               INTEGER REFERENCES entity,
    entity_type             VARCHAR CHECK (entity_type = 'query') DEFAULT 'query',
    FOREIGN KEY (entity_id, entity_type) REFERENCES entity (entity_id, entity_type)
);

CREATE TABLE query_history (
    query_history_id        SERIAL PRIMARY KEY,
    query_id                INTEGER REFERENCES query,
    name                    VARCHAR NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    statement               VARCHAR NOT NULL,
    query_info              JSON NOT NULL DEFAULT '{}',
    modified_at             TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_by             INTEGER REFERENCES user_account
);

CREATE TABLE script (
    script_id               SERIAL PRIMARY KEY,
    entity_id               INTEGER REFERENCES entity,
    entity_type             VARCHAR CHECK (entity_type = 'script') DEFAULT 'script',
    FOREIGN KEY (entity_id, entity_type) REFERENCES entity (entity_id, entity_type)
);

CREATE TABLE script_history (
    script_history_id       SERIAL PRIMARY KEY,
    script_id               INTEGER REFERENCES script,
    name                    VARCHAR NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    script_language         VARCHAR NOT NULL,
    script_text             VARCHAR NOT NULL,
    script_info             JSON NOT NULL DEFAULT '{}',
    modified_at             TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_by             INTEGER REFERENCES user_account
);

CREATE TABLE tag (
    tag_id                  SERIAL PRIMARY KEY,
    name                    VARCHAR NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    tag_info                JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE entity_tag (
    entity_tag_id           SERIAL PRIMARY KEY,
    entity_id               INTEGER REFERENCES entity,
    tag_id                  INTEGER REFERENCES tag
);

CREATE TABLE role (
    role_id                 SERIAL PRIMARY KEY,
    name                    VARCHAR NOT NULL,
    description             VARCHAR NOT NULL DEFAULT '',
    role_info               JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE user_account_role (
    user_account_role_id   SERIAL PRIMARY KEY,
    user_account_id        INTEGER REFERENCES user_account,
    role_id                INTEGER REFERENCES role
);

CREATE TABLE permission (
    permission_id          SERIAL PRIMARY KEY,
    name                   VARCHAR NOT NULL,
    description            VARCHAR NOT NULL DEFAULT '',
    permission_info        JSON NOT NULL DEFAULT '{}'
);

CREATE TABLE role_permission (
    role_permission_id     SERIAL PRIMARY KEY,
    role_id                INTEGER REFERENCES role,
    permission_id          INTEGER REFERENCES permission
);

CREATE TABLE spread_sheet_transaction (
    transaction_id         SERIAL PRIMARY KEY,
    version                VARCHAR NOT NULL DEFAULT '0.1.0',
    action_data            JSON NOT NULL,
    spread_sheet_id        INTEGER REFERENCES spread_sheet,
    made_at                TIMESTAMP NOT NULL DEFAULT NOW(),
    made_by                INTEGER REFERENCES user_account
);

INSERT INTO version (version_update) VALUES ('0.1.0');
