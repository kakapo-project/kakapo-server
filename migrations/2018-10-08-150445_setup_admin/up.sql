
TRUNCATE "scope" CASCADE;
TRUNCATE "user" CASCADE;

INSERT INTO "scope" ("scope_id", "name", "description", "scope_info")
VALUES (1, 'main', '', '{}');

INSERT INTO "user" ("user_id", "username", "password", "email", "display_name")
VALUES (1, 'admin', 'password', 'admin@example.com', 'Admin');


ALTER TABLE "scope" ADD CONSTRAINT "scope_main_is_0_check" CHECK (
    ("scope_id" != 1 AND "name" != 'main') OR
    ("scope_id" = 1 AND "name" = 'main')
);

ALTER TABLE "user" ADD CONSTRAINT "user_admin_is_0_check" CHECK (
    ("user_id" != 1 AND "username" != 'admin') OR
    ("user_id" = 1 AND "username" = 'admin')
);


