
TRUNCATE scope CASCADE;
TRUNCATE user_account CASCADE;

INSERT INTO scope (scope_id, name, description, scope_info)
VALUES (1, 'main', '', '{}');

INSERT INTO user_account (user_account_id, username, password, email)
VALUES (1, 'admin', 'password', 'admin@example.com');


ALTER TABLE scope ADD CONSTRAINT scope_main_is_0_check CHECK (
    (scope_id != 1 AND name != 'main') OR
    (scope_id = 1 AND name = 'main')
);

ALTER TABLE user_account ADD CONSTRAINT user_account_admin_is_0_check CHECK (
    (user_account_id != 1 AND username != 'admin') OR
    (user_account_id = 1 AND username = 'admin')
);


