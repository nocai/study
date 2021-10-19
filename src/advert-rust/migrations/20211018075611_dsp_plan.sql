-- Add migration script here
CREATE TABLE `dsp_plan` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`status` enum('Invalid', "Valid") NOT NULL,
	`name` varchar(255) NOT NULL,
	`budget` BIGINT UNSIGNED NOT NULL,
	`bid_type` enum('CPT', 'CPC', 'CPM') NOT NULL,
	`bid_price` INT UNSIGNED NOT NULL,
	`weekdays` JSON NOT NULL,
	`hours` JSON NOT NULL,
	`begin_at` timestamp NULL,
	`end_at` timestamp NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8

-- ALTER TABLE `dsp_plan` ADD status enum('Invalid', "Valid") NOT NULL;

-- ALTER TABLE `test2`.`table_test` 
-- ADD COLUMN `configs` JSON NOT NULL AFTER `color`;
