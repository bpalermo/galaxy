package service

import (
	"context"
	"github.com/stretchr/testify/assert"
	"google.golang.org/grpc/metadata"
	"testing"
	"time"
)

func TestBaseClient_NewClientAuthContext(t *testing.T) {
	expectedAuthHeader := "bearer 1213"

	inCtx := context.Background()

	md := metadata.New(map[string]string{AuthorizationHeader: expectedAuthHeader})
	inCtx = metadata.NewIncomingContext(inCtx, md)

	client := &BaseClient{
		timeout: time.Second * 10,
	}

	outCtx, cancel, err := client.NewClientAuthContext(inCtx)
	defer cancel()

	assert.NotNil(t, cancel)
	assert.Nil(t, err)

	md, ok := metadata.FromOutgoingContext(outCtx)
	if !ok {
		t.Errorf("Expected MD to exist in ctx, but got none")
	}
	actualHeaders := md.Get(AuthorizationHeader)
	assert.Equal(t, 1, len(actualHeaders))
	assert.Equal(t, expectedAuthHeader, actualHeaders[0])
}
