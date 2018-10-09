
ALTER TABLE user_account DROP CONSTRAINT user_account_admin_is_0_check;
ALTER TABLE scope DROP CONSTRAINT scope_main_is_0_check;

TRUNCATE user_account CASCADE;
TRUNCATE scope CASCADE;
