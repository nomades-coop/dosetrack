FROM rust:1.59

ENV TZ=America/Argentina/Buenos_Aires
RUN ln -snf /usr/share/zoneinfo/$TZ etc/localtime && \
  echo $TZ > /etc/timezone

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
