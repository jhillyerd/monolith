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
    use "nomad-jobspec" {
      jobspec = templatefile("${path.app}/etc/monolith.nomad.tpl")
    }
  }

  release {
    use "nomad-jobspec-canary" {
      groups = ["monolith"]
    }
  }
}
