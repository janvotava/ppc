# PPC [Name TBD] 👮‍♀️

Quick and dirty Google Ads PPC tracker.

## Deploy

I'm using Traefik as a reverse proxy with ACME SSL certificated confirmed using Cloudflare Method.
[Basic Traefik starter configuration can be found here.](https://github.com/janvotava/traefik-proxy-dev)

### Deploying to Docker Swarm

    HOST=redirector.example.com \
    ADMIN_HOST=ppc-admin.example.com \
    ROUTER_PREFIX=${HOST//./-} \
    ADMIN_AUTH=$(htpasswd -nb admin "admin") \
    docker stack deploy -c docker-compose.yml ppc

And set your Tracking template in the Google Ads to:

`https://redirector.example.com/?network={network}&campaignid={campaignid}&adgroup={adgroupid}&ad={creative}&device={device}&devicemodel={devicemodel}&placement={placement}&keyword={keyword}&url={lpurl}`

## Develop

### Usinge VSCode Dev Container

There's bundled Visual Studio Code devcontainer that you can use for local development

or

### Using Docker/Traefik proxy

#### Build

    docker-compose build

#### Running locally

Admin login defaults to `admin:admin`.

    HOST=redirector.example.com \
    ADMIN_HOST=ppc-admin.example.com \
    docker stack deploy -c docker-compose.yml ppc-dev