-- Add migration script here
CREATE TABLE `v_dsp_slot` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),

	`media_id` BIGINT UNSIGNED NULL,
	`slot_id` BIGINT UNSIGNED NULL,
	`slot_type` varchar(255) NULL,

	`os` enum('IOS', 'Android', 'WindowsPhone') NULL,
	`package` varchar(255) NULL,
	`width` INT UNSIGNED NOT NULL,
	`height` INT UNSIGNED NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8