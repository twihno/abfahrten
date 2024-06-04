#!/bin/sh

export PORT=8081
export BASE_URL="/api"
#export STATIONS="de:muc@de:09184:460;de:muc:mvv@ostbahnhof"
 export STATIONS="de:muc:mvv@de:09184:460;de:muc:mvv@de:09184:490"
# export STATIONS="de:muc:mvv@de:09184:460"
# export STATIONS="de:muc:mvv@de:09162:5" # Ostbahnhof
export USE_FIRST_RESULT=false
export FETCH_INTERVAL=180
export LOG_LEVEL=DEBUG

deno run --allow-env=PORT,STATIONS,BASE_URL,USE_FIRST_RESULT,FETCH_INTERVAL,CORS,LOG_LEVEL --allow-read=src/providers/ --allow-net  ./src/main.ts
