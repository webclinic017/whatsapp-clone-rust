resource "helm_release" "argocd" {
  name = "argocd"

  namespace = "argocd"
  create_namespace = true

  repository = "https://argoproj.github.io/argo-helm"
  chart = "argo-cd"
  version = "5.46.8"

  wait = true
}

// Creating the ArgoCD Application Manager
resource "kubernetes_manifest" "argocd_application_manager" {

  manifest = yamldecode(templatefile("${path.module}/argocd-application-manager.yaml", {

    WORKSPACE: var.args.workspace
    BRANCH: var.args.workspace == "dev" ? "dev" : "main"
  }))

  depends_on = [ helm_release.argocd ]
}