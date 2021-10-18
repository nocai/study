-- Add migration script here
CREATE TABLE `strategy_bind_dsp` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`percent` TINYINT UNSIGNED NOT NULL,
	`dsp_provider` varchar(255) NOT NULL,
	`dsp_slot_id` BIGINT UNSIGNED NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8