package main

import (
	"context"
	"errors"
	"os"
	"os/signal"
	"syscall"

	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/adapters"
	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/config"
	"github.com/Archisman-Mridha/whatsapp-clone/backend/outboxer/domain"
	"github.com/charmbracelet/log"

	"golang.org/x/sync/errgroup"
	"gopkg.in/yaml.v2"
)

func main( ) {
	var config config.Config
	configFileContents, err := os.ReadFile("./config.yaml")
	if err != nil {
		log.Fatalf("Error reading file config.yaml: %v", err)}
	if err = yaml.Unmarshal(configFileContents, &config); err != nil {
		log.Fatalf("Error unmarshalling config: %v", err)}

	var (
		// Adapters for sources.
		surrealdbAdapter= adapters.NewSurrealdbAdapter(&config.Source)

		// Adapters for sinks.
		rabbitmqAdapter= adapters.NewRabbitmqAdapter(&config.Sink)

		waitGroup, waitGroupContext= errgroup.WithContext(context.Background( ))

		usecases= domain.NewUsecases(waitGroup, surrealdbAdapter, rabbitmqAdapter)
	)

	usecases.Run( )

	// Listen for system interruption signals to gracefully shut down.
	waitGroup.Go(func( ) error {
		shutdownSignalChan := make(chan os.Signal, 1)
		signal.Notify(shutdownSignalChan, os.Interrupt, syscall.SIGTERM)
		defer signal.Stop(shutdownSignalChan)

		var err error

		select {
			case <-waitGroupContext.Done( ):
				err = waitGroupContext.Err( )

			case shutdownSignal := <-shutdownSignalChan:
				log.Infof("Received program shutdown signal %v", shutdownSignal)
				err = errors.New("received program interruption signal")
		}

		surrealdbAdapter.Disconnect( )
		usecases.CloseChannels( )
		rabbitmqAdapter.Disconnect( )

		return err
	})

	waitGroup.Wait( )
}