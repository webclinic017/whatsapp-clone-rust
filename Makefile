## Generate Bitnami Sealed Secrets from Kubernetes Secret definition files (which have the name
## secret.yaml / *.secret.yaml).
gen-sealed-secrets:
	chmod +x ./scripts/generate-sealed-secrets.sh && \
		./scripts/generate-sealed-secrets.sh

## Generate code for the Kubernetes operator at ./kubernetes/operators
operator-codegen:
	chmod +x ./scripts/operator-codegen.sh && \
		./scripts/operator-codegen.sh