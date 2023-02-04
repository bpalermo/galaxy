package service

import (
	"github.com/rs/zerolog/log"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"time"

	ledger "github.com/bpalermo/galaxy-api/service/ledger/v1"
)

const (
	defaultTimeout = time.Second * 15
)

type LedgerClient struct {
	*BaseClient
	ledger.LedgerServiceClient
}

func NewLedgerClient(target string) *LedgerClient {
	conn, err := grpc.Dial(target, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatal().
			Err(err).
			Msg("failed to dial to ledger server")
		return nil
	}

	return &LedgerClient{
		&BaseClient{
			timeout: defaultTimeout,
			conn:    conn,
		},
		ledger.NewLedgerServiceClient(conn),
	}
}
