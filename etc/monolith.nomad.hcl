variable "image_tag" {
  type = string
}

job "monolith" {
  datacenters = ["skynet"]
  type = "service"

  group "monolith" {
    count = 1

    update {
      canary = 1
      auto_promote = true
      auto_revert = true
      healthy_deadline = "1m"
      progress_deadline = "5m"
    }

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
        name = "HTTP Check"
        type = "http"
        path = "/"
        interval = "10s"
        timeout = "2s"
      }
    }

    task "monolith" {
      driver = "docker"

      config {
        image = "dockreg.bytemonkey.org/monolith:${var.image_tag}"
        ports = ["http"]
        args = ["/local/settings.toml"]
      }

      resources {
        cpu = 200 # MHz
        memory = 32 # MB
      }

      logs {
        max_files = 10
        max_file_size = 5
      }

      template {
        change_mode = "restart"
        data = <<EOT
[server]
port = {{env "NOMAD_PORT_http"}}

[database]
url = "postgresql://monolith:{{key "secrets/monolith/postgres"}}@skynas.home.arpa:54321/monolith"

[mail]
host = "mail.home.arpa"

[home_assistant]
{{- range service "homeassistant-ui" }}
url = "http://{{ .Address }}:{{ .Port }}"
{{- end }}
token = "{{key "secrets/monolith/homeassistant"}}"
EOT

        destination = "local/settings.toml"
      }
    }
  }
}
