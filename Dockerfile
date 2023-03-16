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

# Frontend
ARG AUTH0_DOMAIN
ARG AUTH0_CLIENT_ID
ARG AUTH0_AUDIENCE
ARG API_SERVER_URL

RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt -y install nodejs
RUN npm install -g npm 

RUN mkdir /usr/src/myapp
COPY . /usr/src/myapp

WORKDIR /usr/src/myapp/frontend/dosetrack
RUN npm install
RUN npm run build

WORKDIR /usr/src/myapp/backend/dosetrack-ws

RUN rustup component add rust-analysis
RUN rustup component add rust-src
RUN rustup component add rls
RUN rustup component add rustfmt

RUN cargo install cargo-watch
RUN cargo build --release

RUN echo 'alias l="ls -CF"' >> ~/.bashrc 
RUN echo 'alias la="ls -A"' >> ~/.bashrc 
RUN echo 'alias ll="ls -halF"' >> ~/.bashrc 
RUN echo 'alias ls="ls --color=auto"' >> ~/.bashrc 

EXPOSE 3000

CMD /usr/src/myapp/target/release/dosetrack-api
