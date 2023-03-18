FROM node:19-bullseye

RUN chsh -s /bin/bash && \
  sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b /usr/local/bin && \
  npm i -g pnpm
