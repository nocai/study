-- Add migration script here
CREATE TABLE `slot` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`media_id` BIGINT UNSIGNED NOT NULL,
	`name` varchar(45) NOT NULL,
	`refresh_type` enum('None', 'CurrentTime', 'CurrentDay') NOT NULL,
	`open_screen` boolean NOT NULL,
	`times` TINYINT UNSIGNED NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8