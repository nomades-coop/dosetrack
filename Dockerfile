FROM rust:latest

ENV TZ=America/Argentina/Buenos_Aires
RUN ln -snf /usr/share/zoneinfo/$TZ etc/localtime && \
  echo $TZ > /etc/timezone

ARG USERNAME=dosetrack
ARG USER_UID=1000
ARG USER_GID=1000

# Backend
ARG ROCKET_LOG_LEVEL=critical
ARG ROCKET_SECRET_KEY
ARG ROCKET_ADDRESS
ARG ROCKET_PORT
ARG MDB_URL

ENV ROCKET_LOG_LEVEL=${ROCKET_LOG_LEVEL}
ENV ROCKET_SECRET_KEY=${ROCKET_SECRET_KEY}
ENV ROCKET_ADDRESS=${ROCKET_ADDRESS}
ENV ROCKET_PORT=${ROCKET_PORT}
ENV MDB_URL=${MDB_URL}

# Frontend
ARG AUTH0_DOMAIN
ARG AUTH0_CLIENT_ID
ARG AUTH0_AUDIENCE
ARG API_SERVER_URL
ARG FRONTEND_PATH

ENV AUTH0_DOMAIN=${AUTH0_DOMAIN}
ENV AUTH0_CLIENT_ID=${AUTH0_CLIENT_ID}
ENV AUTH0_AUDIENCE=${AUTH0_AUDIENCE}
ENV API_SERVER_URL=${API_SERVER_URL}
ENV FRONTEND_PATH=${FRONTEND_PATH}

RUN curl -sL https://deb.nodesource.com/setup_16.x | bash - && \
  apt -y install nodejs && \
  npm install -g npm 

RUN mkdir -p /app
COPY . /app

WORKDIR /app/frontend/dosetrack
RUN npm install && npm run build


WORKDIR /app/

RUN wget -q https://github.com/nomades-coop/dosetrack/releases/download/v0.1.0/dosetrack-api && \
  chmod +x dosetrack-api

RUN echo 'alias l="ls -CF"' >> ~/.bashrc && \
  echo 'alias la="ls -A"' >> ~/.bashrc && \
  echo 'alias ll="ls -halF"' >> ~/.bashrc && \
  echo 'alias ls="ls --color=auto"' >> ~/.bashrc 

EXPOSE 80
EXPOSE 443

CMD /app/dosetrack-api
