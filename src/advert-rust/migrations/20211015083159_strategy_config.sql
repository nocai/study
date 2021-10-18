-- Add migration script here

CREATE TABLE `strategy_config` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`strategy_id` BIGINT UNSIGNED NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8
