// Code generated by sqlc. DO NOT EDIT.
// versions:
//   sqlc v1.17.2

package db

import (
	"database/sql"
	"time"
)

type Account struct {
	ID        []byte       `json:"id"`
	OwnerID   []byte       `json:"owner_id"`
	Currency  string       `json:"currency"`
	Balance   string       `json:"balance"`
	UpdatedAt sql.NullTime `json:"updated_at"`
	CreatedAt time.Time    `json:"created_at"`
}