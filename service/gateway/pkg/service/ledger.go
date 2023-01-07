package service

import (
	"github.com/rs/zerolog/log"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"time"

	ledger "github.com/bpalermo/galaxy-api/service/ledger/v1"
)

type LedgerClient struct {
	ledger.LedgerServiceClient
	BaseService
}

func NewLedgerClient(target string) *LedgerClient {
	conn, err := grpc.Dial(target, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatal().
			Err(err).
			Msg("failed to dial to ledger server")
	}

	return &LedgerClient{
		LedgerServiceClient: ledger.NewLedgerServiceClient(conn),
		BaseService: BaseService{
			conn:    conn,
			timeout: 15 * time.Second,
		},
	}
}

func (l *LedgerClient) Shutdown() error {
	err := l.conn.Close()
	if err != nil {
		log.Fatal().Err(err).Msg("failed to close client connection")
		return err
	}

	return nil
}
