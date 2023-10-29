// Atlas is a language-independent tool for managing and migrating database schemas using modern
// DevOps principles.
// We will use their declarative workflow -  Similar to Terraform, Atlas compares the current state
// of the database to the desired state. It then generates and executes a migration plan to
// transition the database to its desired state.
resource "helm_release" "atlas" {
  name = "atlas"

  namespace = "atlas"
  create_namespace = true

  repository = "oci://ghcr.io/ariga/charts"
  chart = "atlas-operator"

  wait = true
}