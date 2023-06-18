FROM docker.io/denoland/deno:alpine as builder
RUN mkdir /abfahrten
COPY --chown=denouser:denogroup src /abfahrten/src
COPY --chown=denouser:denogroup deps.ts /abfahrten/deps.ts
WORKDIR /abfahrten
RUN deno vendor deps.ts

FROM docker.io/denoland/deno:alpine

RUN addgroup -S denogroup && adduser -S denouser -G denogroup && install -d -m 0755 -o denouser -g denogroup /abfahrten && install -d -m 0755 -o denouser -g denogroup /deno-dir
USER denouser
WORKDIR /abfahrten

COPY --chown=denouser:denogroup --from=builder /abfahrten /abfahrten

CMD ["deno","run","--no-remote","--import-map=vendor/import_map.json","--allow-env=PORT,STATIONS,BASE_URL,USE_FIRST_RESULT,FETCH_INTERVAL,CORS", "--allow-read=src/providers/","--allow-net","src/main.ts"]
