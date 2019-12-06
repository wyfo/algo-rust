FROM ubuntu:18.04

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y vim sudo curl linux-tools-generic gcc valgrind kcachegrind
RUN rm -f /usr/bin/perf
RUN ln -s /usr/lib/linux-tools/4.15.0-42-generic/perf /usr/bin/perf
RUN ln -s ~/.cargo/bin/cargo /usr/bin/cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
