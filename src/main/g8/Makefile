
NAME=\$(shell sed -E -n 's/name = "(.*)"/\1/p' Cargo.toml | head -1)
VERSION=\$(shell cat VERSION)
GIT_BRANCH=\$(shell git rev-parse --abbrev-ref HEAD)
GIT_REV=\$(shell git rev-parse --short HEAD)
REV_FILE=\$(shell cat rev.log 2> /dev/null)

all: version


src/build.rs:
	@@if [ "\$(GIT_BRANCH) \$(GIT_REV) \$(VERSION)" != "\$(REV_FILE)" ]; then \
		echo \$(GIT_BRANCH) \$(GIT_REV) \$(VERSION) > rev.log; \
		echo "Generating build.rs ..."; \
		echo "// this is auto-generated file, don't edit this by your dirty hands!!" > src/build.rs; \
		echo "pub const GIT_REV:&'static str = \"\$(GIT_REV) (\$(GIT_BRANCH))\";" >> src/build.rs; \
		echo "pub const VERSION:&'static str =  \"\$(VERSION)\";" >> src/build.rs; \
	fi

init-dev:
	@@if [ -f src/build.rs ]; then \
		echo "already initialized!"; \
	else \
		echo "initializing..."; \
		make src/build.rs; \
	fi;

version: src/build.rs
	@@sed -i.bak 's/version = ".*" # auto generated do not edit by hand/version = "\$(VERSION)" # auto generated do not edit by hand/' Cargo.toml

dist:
	@@echo \$(NAME)
	git archive --format=tar.gz --prefix=\$(NAME)-src/ master > \$(NAME)-src-master.tar.gz

clean:

.PHONY: all version clean init-dev src/build.rs dist
