-- Add migration script here fro creating the user table {postgres}
-- -- This is a placeholder for the migration script to create the user table
-- CREATE TABLE "users" (
--     "id" BIGSERIAL PRIMARY KEY,
--     "first_name" VARCHAR(255) NOT NULL,
--     "last_name" VARCHAR(255) NOT NULL,
--     "email" VARCHAR(255) NOT NULL UNIQUE,
--     "password" VARCHAR(255) NOT NULL,
--     "balance" BIGINT NOT NULL DEFAULT 0,
--     "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );

-- CREATE INDEX "idx_user_email" ON "users" ("email");
-- -- Add any additional SQL commands needed for the migration



-- Add migration script here fro creating the user table {mysql}
-- -- This is a placeholder for the migration script to create the user table
CREATE TABLE `users` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `email` varchar(255) NOT NULL,
  `firstname` varchar(255) NOT NULL,
  `lastname` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  `balance` BIGINT UNSIGNED NOT NULL DEFAULT 0,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
);