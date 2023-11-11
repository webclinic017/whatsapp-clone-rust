package adapters

import (
	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/config"
	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/domain"
	"github.com/charmbracelet/log"
	"github.com/surrealdb/surrealdb.go"
)

type SurrealdbAdapter struct {
	connection *surrealdb.DB
}

// NewSurrealdbAdapter establishes connection with the Surrealdb database, constructs an instance of
// the SurrealdbAdapter struct and returns it.
func NewSurrealdbAdapter(config *config.SurrealdbConfig) *SurrealdbAdapter {

	connection, err := surrealdb.New(config.Uri)
	if err != nil {
		log.Fatalf("Error connecting to Surrealdb : %v", err)}

	_, err= connection.Signin(map[string]interface{}{
		"user": config.User,
		"pass": config.Password,
	})
	if err != nil {
		log.Fatalf("Error signing in to Surrealdb : %v", err)}

	if _, err := connection.Use(config.Namespace, config.Database); err != nil {
		log.Fatalf("Error using namespace / database : %v", err)
	}

	log.Info("Connected to Surrealdb")

	return &SurrealdbAdapter { connection }
}

// Disconnect closes the underlying connection to Surrealdb.
func(s *SurrealdbAdapter) Disconnect( ) {
	s.connection.Close( )
}

type Row struct {
	Id string				`json:"id"`
	Message []byte	`json:"message"`
	Locked bool 		`json:"locked"`
	LockedAt bool 	`json:"locked_at"`
}

func(s *SurrealdbAdapter) GetMessageBatch( ) domain.MessageBatch {
	query := `
		LET $records= (SELECT id FROM outboxers WHERE locked= false);
		UPDATE $records SET locked= true, locked_at= time::now( );
	`

	result, err := s.connection.Query(query, map[string]interface{}{ })
	if err != nil {
		log.Fatalf("Error querying messages from Surrealdb : %v", err)}

	type QueryOutput= [2]interface{ }
	var queryOutput QueryOutput
	surrealdb.Unmarshal(result, queryOutput)

	type UpdateQueryOutput= []Row
	var updateQueryOutput UpdateQueryOutput
	surrealdb.Unmarshal(queryOutput[0], updateQueryOutput)

	var messageBatch domain.MessageBatch
	for _, row := range updateQueryOutput {
		messageBatch = append(messageBatch, &domain.Message {
			Id: row.Id,
			Body: row.Message,
		})
	}
	return messageBatch
}

func(s *SurrealdbAdapter) DeleteMessages( ) { }

func(s *SurrealdbAdapter) UnlockMessages( ) { }