job "monolith" {
  datacenters = ["skynet"]
  type = "service"

  meta = {
    # Allow Waypoint to detect release URL.
    "waypoint.hashicorp.com/release_url" = "https://monolith.bytemonkey.org"
  }

  group "monolith" {
    count = 1

    network {
      port "http" { to = 3000 }
    }

    service {
      name = "monolith-http"
      port = "http"

      tags = [
        "http",
        "traefik.enable=true",
        "traefik.http.routers.monolith-http.entrypoints=websecure",
        "traefik.http.routers.monolith-http.rule=Host(`monolith.bytemonkey.org`)",
        "traefik.http.routers.monolith-http.tls.certresolver=letsencrypt",
      ]

      check {
        name = "Monolith HTTP Check"
        type = "http"
        path = "/"
        interval = "10s"
        timeout = "2s"
      }
    }

    task "monolith" {
      driver = "docker"

      config {
        image = "${artifact.image}:${artifact.tag}"
        ports = ["http"]

        volumes = [
          "../alloc/data/monolith_storage:/storage"
        ]
      }

      env {
        %{ for k,v in entrypoint.env ~}
        ${k} = "${v}"
        %{ endfor ~}

        # Ensure we set PORT for the URL service. This is only necessary
        # if we want the URL service to function.
        PORT = 3000
      }

      resources {
        cpu = 200 # MHz
        memory = 32 # MB
      }

      logs {
        max_files = 10
        max_file_size = 5
      }
    }
  }
}
