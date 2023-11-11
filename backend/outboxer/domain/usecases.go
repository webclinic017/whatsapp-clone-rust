package domain

import (
	"time"

	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/utils"
	"github.com/charmbracelet/log"
	"golang.org/x/sync/errgroup"
)

type (
	Message struct {
		Id string
		Body []byte
	}

	MessageBatch= []*Message

	Db interface {
		// GetMessageBatch queries the outbox table. It searches for a batch of rows which are unlocked.
		// It then extracts the message from each of those rows. Batch of those messages is returned.
		GetMessageBatch( ) MessageBatch

		// DeleteMessages
		DeleteMessages( )

		// UnlockMessages
		UnlockMessages( )
	}

	Mq interface {
		// PublishMessage sends a message to the message queue.
		PublishMessage(message *Message) error
	}

	Usecases struct {
		waitGroup *errgroup.Group

		db Db
		mq Mq

		// Messages in the message batches returned by db.GetMessageBatch will be pushed into this Go
		// channel. The outboxer will then try to publish the messages to the message queue.
		messagesToBePublishedChan chan *Message

		// A message may or may not be successfully published to the message queue.

		// If successfully published, id of the message will be pushed to this GoLang channel. The
		// message will then be deleted from the database.
		messagesSuccessfullyPublishedChan chan string

		// Otherwise it's id will be pushed to this GoLang channel. It needs to be unlocked, leaving a
		// scope for future retrials.
		// NOTE - We can be more fancy and record the number of retries for a message. If the number of
		// retries cross a certain threshold, then the outboxer can just ignore it.
		messagesFailedToBePublishChan chan string
	}
)

// NewUsecases creates a new instance of Usecases and returns it.
func NewUsecases(waitGroup *errgroup.Group, db Db, mq Mq) *Usecases {
	return &Usecases {
		waitGroup: waitGroup,

		db: db,
		mq: mq,

		messagesToBePublishedChan: make(chan *Message),

		messagesSuccessfullyPublishedChan: make(chan string),
		messagesFailedToBePublishChan: make(chan string),
	}
}

// CloseChannels closes the GoLang channels contained by the Usecases struct, for gracefull
// shutdown.
func(u *Usecases) CloseChannels( ) {
	close(u.messagesToBePublishedChan)

	close(u.messagesSuccessfullyPublishedChan)
	close(u.messagesFailedToBePublishChan)
}

// Run contains the main business logic of outboxing message batches from the database to the
// message queue.
func(u *Usecases) Run( ) {
	timePeriod := 3 * time.Second

	utils.RunFnPeriodically(u.waitGroup, timePeriod, func( ) {
		for _, message := range u.db.GetMessageBatch( ) {
			u.messagesToBePublishedChan <- message
		}
	})

	utils.RunFnPeriodically(u.waitGroup, timePeriod, func( ) {
		for message := range u.messagesToBePublishedChan {
			id := message.Id

			err := u.mq.PublishMessage(message)
			switch err {
				case nil:
					u.messagesSuccessfullyPublishedChan <- id

				default:
					log.Errorf("Error sending message to the message queue")
					u.messagesFailedToBePublishChan <- id
			}
		}
	})
}