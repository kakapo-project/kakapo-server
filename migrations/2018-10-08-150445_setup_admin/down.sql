
ALTER TABLE "user" DROP CONSTRAINT "user_admin_is_0_check";
ALTER TABLE "scope" DROP CONSTRAINT "scope_main_is_0_check";

TRUNCATE "user" CASCADE;
TRUNCATE "scope" CASCADE;
