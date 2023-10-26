variable "args" {
  type = object({
    digitalocean = object({
      token = string
      region = string
    })
  })
}

locals {
  droplet = {
    // 2 GB RAM
    // 2 AMD CPUs
    // 60GB NVMe SSD as the boot disk.
    size = "s-2vcpu-2gb-amd"
  }
}

resource "local_file" "kubeconfig" {
  filename = "./outputs/kubeconfig"

  // Only owner can perform read, write and execute operations on the file.
  file_permission = "700"

  content = digitalocean_kubernetes_cluster.default.kube_config.0.raw_config
}