-- Add migration script here
CREATE TABLE `strategy_trigger_rule` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`config_id` BIGINT UNSIGNED NOT NULL,
	`key` varchar(255) NOT NULL,
	`value` varchar(255) NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8