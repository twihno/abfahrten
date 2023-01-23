#!/bin/sh

export PORT=8080
export BASE_URL=/api
export STATIONS="de:muc@de:09184:460;de:muc:mvv@garching"

deno run --allow-env=PORT,STATIONS,BASE_URL --allow-read=src/providers/ --allow-net=www.mvv-muenchen.de  ./src/main.ts
