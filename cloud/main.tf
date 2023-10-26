terraform {
  required_version = ">= 1.5.3"

  backend "local" { }

  required_providers {
    digitalocean = {
      source = "digitalocean/digitalocean"
      version = "2.31.0"
    }
  }
}

provider "digitalocean" {
  token = var.args.digitalocean.token
}

// A Virtual Private Cloud (VPC) is a private network interface for collections of DigitalOcean
// resources. VPC networks provide a more secure connection between resources because the network is
// inaccessible from the public internet and other VPC networks. Traffic within a VPC network doesn’t
// count against bandwidth usage.
resource "digitalocean_vpc" "default" {
  name = "Main"
  region = var.args.digitalocean.region

  ip_range = "10.0.0.0/24"
}

resource "digitalocean_kubernetes_cluster" "default" {
  name = "main"
  region = var.args.digitalocean.region

  vpc_uuid = digitalocean_vpc.default.id

  version = "1.28.2-do.0"
  auto_upgrade = false
  maintenance_policy {
    day = "thursday"
    start_time = "00:00"
  }

  ha = false

  node_pool {
    name = "default"

    auto_scale = true
    min_nodes = 1
    max_nodes = 3

    // DigitalOcean Droplets are Linux-based virtual machines (VMs) that run on top of virtualized
    // hardware.
    // The size of DigitalOcean Droplets to use as worker nodes.
    size = local.droplet.size

    labels = {
      "node_type" = "general_purpose"
    }
  }

  destroy_all_associated_resources = true
}