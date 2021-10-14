-- Add migration script here
CREATE TABLE `role` (
	`id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
	`created_at` timestamp NOT NULL DEFAULT current_timestamp(),
	`updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
	`name` varchar(45) NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8;