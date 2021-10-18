-- Add migration script here
CREATE TABLE `dsp_slot` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`name` varchar(255) NOT NULL,
	`accept_ad_type` JSON NOT NULL,
	`min_price_cpt` INT UNSIGNED NOT NULL,
	`min_price_cpm` INT UNSIGNED NOT NULL,
	`min_price_cpc` INT UNSIGNED NOT NULL,
	`width` INT UNSIGNED NOT NULL,
	`height` INT UNSIGNED NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8