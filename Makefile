## Generate Bitnami Sealed Secrets from Kubernetes Secret definition files (which have the name
## secret.yaml / *.secret.yaml).
gen-sealed-secrets:
	chmod +x ./scripts/generate-sealed-secrets.sh && \
		./scripts/generate-sealed-secrets.sh

## Generate code for the Kubernetes operator at ./kubernetes/operators
operator-codegen:
	chmod +x ./scripts/operator-codegen.sh && \
		./scripts/operator-codegen.sh

## Generate GoLang types from protobuf definitions.
protoc-gen-go:
	rm -rf ./backend/outboxer/generated/protoc && \
		mkdir -p ./backend/outboxer/generated/protoc
	protoc \
		--experimental_allow_proto3_optional \
		--go_out=./backend/outboxer/generated/protoc --go_opt=paths=source_relative \
		--proto_path=./protos \
		./protos/events.proto
	cd ./backend/outboxer && \
		go mod tidy
	go work sync

## Generate a token using which we can signin into the Kiali dashboard.
get-kiali-token:
	kubectl -n istio-system create token kiali-service-account