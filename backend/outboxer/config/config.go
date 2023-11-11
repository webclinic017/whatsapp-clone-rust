package config

type (
	Config struct {
		// Source currently represents an outboxer table named 'outboxer' in a Surrealdb database.
		Source Source `yaml:"source,omitempty"`

		// Sink currently represents a RabbitMQ queue.
		Sink Sink `yaml:"sink,omitempty"`
	}

	Source struct {
		Uri 			string `yaml:"uri,omitempty"`
		Namespace string `yaml:"namespace,omitempty"`
		Database 	string `yaml:"database,omitempty"`
		User 			string `yaml:"user,omitempty"`
		Password 	string `yaml:"password,omitempty"`
	}
	SurrealdbConfig= Source

	Sink struct {
		Uri   string `yaml:"uri"`
		Queue string `yaml:"queue"`
	}
	RabbitmqConfig= Sink
)