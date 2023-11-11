package microservices

## These policies are applied to only the microservices namespaces.

## Only Kubernetes Services of type ClusterIP can be created.
block_non_cluster_ip_type_services[msg] {
  input.kind == "Service"
  input.metadata.namespace == "microservices"

  input.spec.type != "ClusterIP"

  msg := sprintf("Only Kubernetes Services of type ClusterIP can be created in the microservices namespace")
}

## Only container images from the 'ghcr.io/archisman-mridha' container registry are allowed.
block_other_container_registries[msg] {
  input.kind == "Pod"
  input.metadata.namespace == "microservices"

  container := input.spec.containers[_]

  not startswith(container.image, "ghcr.io/archisman-mridha/")

  msg := sprintf("Only containers using images from the 'ghcr.io/archisman-mridha' container registry are allowed in the 'microservices' namespace.")
}

## Kubernetes Deployments in the 'microservice' namespace cannot have more than 200 pod replicas.
limit_replica_count[msg] {
  input.kind == "Deployment"
  input.metadata.namespace == "microservices"

  input.spec.replicas <= 200

  msg := sprintf("Deployments in the 'microservices' namespace can have a replica count of up to 200.")
}