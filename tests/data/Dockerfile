# 17 lines 7 code 3 comments 7 blanks

FROM netbsd:7.0.2

MAINTAINER Somebody version: 2.2

RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- -y

# this part is important
VOLUME ["/project"]
WORKDIR "/project"

RUN sh -c 'echo "Hello World" > /dev/null'
RUN cargo install tokei # not counted

# now you do your part

