-- Add migration script here
CREATE TABLE `strategy_trigger_target` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`config_id` BIGINT UNSIGNED NOT NULL,
	`percent` TINYINT UNSIGNED NOT NULL,
	`group1` varchar(255) NOT NULL,
	`group2` varchar(255) NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8