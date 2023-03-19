CREATE TABLE
  `account`
(
  `id`         BINARY(16) PRIMARY KEY NOT NULL,
  `owner_id`   BINARY(16)             NOT NULL,
  `currency`   CHAR(3)                NOT NULL,
  `balance`    DECIMAL(14, 4)         NOT NULL DEFAULT 0,
  `updated_at` DATETIME(6)            NULL,
  `created_at` DATETIME(6)            NOT NULL,
  INDEX `owner_id_idx` (`owner_id` ASC),
  INDEX `updated_at_idx` (`updated_at` ASC),
  INDEX `created_at_asc_idx` (`created_at` ASC),
  INDEX `created_at_desc_idx` (`created_at` DESC )
);
