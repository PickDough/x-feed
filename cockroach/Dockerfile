# Stage 1 - generate certs
FROM cockroachdb/cockroach:latest AS generator

RUN mkdir -pv /tmp/certs/ca /tmp/certs/roach-1 /tmp/certs/roach-2 /tmp/certs/roach-3 /tmp/safe /tmp/certs/client

RUN cockroach cert create-ca --certs-dir=/tmp/certs/ca --ca-key=/tmp/safe/ca.key \
    && cp -v /tmp/certs/ca/ca.crt /tmp/certs/client \
    && cp -v /tmp/certs/ca/ca.crt /tmp/certs/roach-1 \
    && cp -v /tmp/certs/ca/ca.crt /tmp/certs/roach-2 \
    && cp -v /tmp/certs/ca/ca.crt /tmp/certs/roach-3

RUN ./cockroach cert create-client root --certs-dir=/tmp/certs/client --ca-key=/tmp/safe/ca.key --also-generate-pkcs8-key
RUN ./cockroach cert create-client roach --certs-dir=/tmp/certs/client --ca-key=/tmp/safe/ca.key --also-generate-pkcs8-key

RUN cp -v /tmp/certs/client/client.* /tmp/certs/roach-1 \
    && cp -v /tmp/certs/client/client.* /tmp/certs/roach-2 \
    && cp -v /tmp/certs/client/client.* /tmp/certs/roach-3

RUN cockroach cert create-node roach-1 --certs-dir=/tmp/certs/roach-1 --ca-key=/tmp/safe/ca.key
RUN cockroach cert create-node roach-2 --certs-dir=/tmp/certs/roach-2 --ca-key=/tmp/safe/ca.key
RUN cockroach cert create-node roach-3 --certs-dir=/tmp/certs/roach-3 --ca-key=/tmp/safe/ca.key

# Stage 2 - share certs

FROM alpine:3

RUN mkdir -pv /certs/roach-1 /certs/roach-2 /certs/roach-3 /certs/client

COPY --from=generator  /tmp/certs/roach-1/* /certs/roach-1/
COPY --from=generator  /tmp/certs/roach-2/* /certs/roach-2/
COPY --from=generator  /tmp/certs/roach-3/* /certs/roach-3/
COPY --from=generator  /tmp/certs/client/* /certs/client/

CMD tail -f /dev/null