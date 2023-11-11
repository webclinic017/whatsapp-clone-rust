package utils

import (
	"time"

	"golang.org/x/sync/errgroup"
)

// RunFnPeriodically runs the given funcion (in a separate go-routine) periodically with the given
// time period.
func RunFnPeriodically(waitGroup *errgroup.Group, timePeriod time.Duration, fn func( )) {
	waitGroup.Go(func( ) error {
		ticker := time.NewTicker(timePeriod)
		defer ticker.Stop( )

		for range ticker.C {
			fn( )}

		return nil
	})
}