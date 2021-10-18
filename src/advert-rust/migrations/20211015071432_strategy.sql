-- Add migration script here
CREATE TABLE `strategy` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`slot_id` BIGINT UNSIGNED NOT NULL,
	`media_id` BIGINT UNSIGNED NOT NULL,
	`name` varchar(45) NOT NULL,
	`status` enum('Invalid', 'Valid') NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8