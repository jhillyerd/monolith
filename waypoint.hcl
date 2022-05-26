project = "monolith"

# An application to deploy.
app "monolith" {
  build {
    use "docker" {}
  }

  deploy {
    use "docker" {}
  }
}
