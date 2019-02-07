
TRUNCATE "scope" CASCADE;
TRUNCATE "user" CASCADE;

INSERT INTO "scope" ("name", "description", "scope_info")
VALUES ('main', '', '{}');

INSERT INTO "user" ("username", "password", "email", "display_name")
VALUES ('admin', 'password', 'admin@example.com', 'Admin');


ALTER TABLE "scope" ADD CONSTRAINT "scope_main_is_0_check" CHECK (
    ("scope_id" != 1 AND "name" != 'main') OR
    ("scope_id" = 1 AND "name" = 'main')
);

ALTER TABLE "user" ADD CONSTRAINT "user_admin_is_0_check" CHECK (
    ("user_id" != 1 AND "username" != 'admin') OR
    ("user_id" = 1 AND "username" = 'admin')
);


