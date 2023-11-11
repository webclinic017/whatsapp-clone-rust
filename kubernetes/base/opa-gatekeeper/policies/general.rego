package general

## These policies are applied to all namespaces.

## Blacklist the 'latest' tag. Container images must use specific version tags.
deny_latest_container_tag[msg] {
  input.kind == "Pod"

  container := input.spec.containers[_]
  endsWith(container.image, ":latest")

  msg := sprintf("Container image '%v' in Pod '%v' is using the ':latest' tag. Please use a specific version tag.", [container.image, input.metadata.name])
}

## Resource limits must be specified for all containers in Kubernetes Pods.
deny_pods_without_limits[msg] {
  input.kind == "Pod"

  container := input.spec.containers[_]

  not container.resources.limits

  msg := sprintf("Pods must have resource limits set for all containers")
}

## Kubernetes Custom Resources of type 'whatappclone.io/Application' must be created in the
## 'microservices' namespace.
allow_application_type_resources_in_only_microservices_namespaces[msg] {
  input.kind == "Application"
  input.apiVersion == "whatsappclone.io/v1"

  input.metadata.namespace != "microservices"

  msg := sprintf("Resources of type 'Application' belonging to 'whatsappclone.io' API must be created in the 'microservices' namespace.")
}

## Persistent Volumes (PV) and Persistent Volume Claims (PVC) of size greater than 5 GB are not
## allowed.
limit_pv_or_pvc_size[msg] {
  (input.kind == "PersistentVolume" or input.kind == "PersistentVolumeClaim") and

  input.spec.capacity.storage > 5 * 1024 * 1024 * 1024

  msg := sprintf("Persistent Volumes (PV) and Persistent Volume Claims (PVC) must have a size of 5 GB or less.")
}

## Kubernetes Deployments in namespaces other than the 'microservice' namespace cannot have more
## than 5 pod replicas.
limit_replica_count[msg] {
  input.kind == "Deployment"
  input.metadata.namespace != "microservices"

  input.spec.replicas > 10

  msg := sprintf("Deployments in namespaces other than 'microservices' cannot have more than 10 replicas.")
}