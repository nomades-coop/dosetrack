FROM rust:1.59

ENV TZ=America/Argentina/Buenos_Aires
RUN ln -snf /usr/share/zoneinfo/$TZ etc/localtime && \
  echo $TZ > /etc/timezone

RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt -y install nodejs
RUN npm install -g npm@8.8.0 


RUN wget -qO - https://www.mongodb.org/static/pgp/server-5.0.asc | apt-key add -
RUN echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu focal/mongodb-org/5.0 multiverse" | tee /etc/apt/sources.list.d/mongodb-org-5.0.list
RUN apt update
RUN apt install -y mongodb-mongosh

COPY ./backend/dosetrack-ws/. /usr/src/myapp

WORKDIR /usr/src/myapp

RUN rustup component add rust-analysis
RUN rustup component add rust-src
RUN rustup component add rls

RUN cargo install cargo-edit
RUN cargo install cargo-watch
RUN cargo build


RUN echo 'alias l="ls -CF"' >> ~/.bashrc 
RUN echo 'alias la="ls -A"' >> ~/.bashrc 
RUN echo 'alias ll="ls -halF"' >> ~/.bashrc 
RUN echo 'alias ls="ls --color=auto"' >> ~/.bashrc 

EXPOSE 8000

CMD bash
