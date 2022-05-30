project = "monolith"

# An application to deploy.
app "monolith" {
  build {
    use "docker" {}

    registry {
      use "docker" {
        image = "dockreg.bytemonkey.org/monolith"
        tag = "edge"
      }
    }
  }

  deploy {
    use "nomad" {
      datacenter = "skynet"
    }
  }
}
