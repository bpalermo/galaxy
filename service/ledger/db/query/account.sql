-- name: GetAccountById :one
SELECT *
FROM `account`
WHERE `id` = $1
LIMIT 1;

-- name: GetAccountsByOwner :many
SELECT *
FROM `account`
WHERE `owner_id` = $1;

-- name: GetAccountsByCurrencyAndOwnerId :many
SELECT *
FROM `account`
WHERE `owner_id` = $1
  AND `currency` = $2;

-- name: DeleteAccount :exec
DELETE
FROM `account`
WHERE `id` = $1;

-- name: CreateAccount :execresult
INSERT INTO `account` (
  `owner_id`,
  `currency`,
  `balance`,
  `created_at`
) VALUES (
  ?,
  ?,
  0,
  UTC_TIMESTAMP() + 6
)
