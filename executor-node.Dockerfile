FROM node:18 as node-base

FROM fpr-executor-base:latest

COPY --from=node-base /usr/local/lib/ /usr/local/lib/
COPY --from=node-base /usr/local/bin/node /usr/local/bin/node

COPY --from=node-base /etc/ld.so.cache /etc/ld.so.cache
