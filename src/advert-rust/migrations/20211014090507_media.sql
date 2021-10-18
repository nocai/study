-- Add migration script here
CREATE TABLE `media` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`name` varchar(45) NOT NULL,
	`desc` varchar(45) NULL,
	`media_type` enum('APP', 'WAP') NOT NULL,
	`site_url` varchar(45) NULL,
	`os` enum('IOS', 'Android', 'WP') NULL,
	`package_name` varchar(45) NULL,
	`down_load_url` varchar(1024) NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8;