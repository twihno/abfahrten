FROM docker.io/denoland/deno:alpine as builder
RUN mkdir /abfahrten
COPY --chown=denouser:denogroup src /abfahrten/src
COPY --chown=denouser:denogroup deps.ts /abfahrten/deps.ts
WORKDIR /abfahrten
RUN deno vendor deps.ts

FROM docker.io/denoland/deno:alpine

LABEL org.opencontainers.image.source=https://github.com/twihno/abfahrten
LABEL org.opencontainers.image.description="Simple application to display the departures of a public transport stop"
LABEL org.opencontainers.image.licenses=MIT

USER deno
WORKDIR /deno-dir
COPY --chown=deno:deno --from=builder /abfahrten /deno-dir

CMD ["run","--no-remote","--import-map=vendor/import_map.json","--allow-env=PORT,STATIONS,BASE_URL,USE_FIRST_RESULT,FETCH_INTERVAL,CORS", "--allow-read=src/providers/","--allow-net","src/main.ts"]
