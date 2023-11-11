package adapters

import (
	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/config"
	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/domain"
	"github.com/charmbracelet/log"
	"github.com/streadway/amqp"
)

type RabbitmqAdapter struct {
	connection *amqp.Connection
	channel *amqp.Channel
	queue string
}

// NewRabbitmqAdapter establishes a Rabbitmq connection, constructs an instance of the
// RabbitmqAdapter struct and returns it.
func NewRabbitmqAdapter(config *config.RabbitmqConfig) *RabbitmqAdapter {
	connection, err := amqp.Dial(config.Uri)
	if err != nil {
		log.Fatalf("Error connecting to RabbitMQ : %v", err)}

	channel, err := connection.Channel( )
	if err != nil {
		log.Fatalf("Error creating channel in RabbitMQ : %v", err)}

	_, err = channel.QueueDeclare(config.Queue, true, false, false, false, nil)
	if err != nil {
		log.Fatalf("Error declaring RabbitMQ queue : %v", err)}

	log.Info("Connected to RabbitMQ")

	return &RabbitmqAdapter{
		connection: connection,
		channel: channel,
		queue: config.Queue,
	}
}

// Disconnect closes the underlying connection to Rabbitmq. All resources related to the connection
// gets cleaned up.
func (r *RabbitmqAdapter) Disconnect( ) {
	if err := r.connection.Close( ); err != nil {
		log.Errorf("Error closing Rabbitmq connection : %v", err)
	}
}

func(r *RabbitmqAdapter) PublishMessage(message *domain.Message) error {
	err := r.channel.Publish("", r.queue, true, false, amqp.Publishing{ Body: message.Body })
	if err != nil {
		log.Printf("Error trying to publish message to Rabbitmq : %v", err)}

	return err
}