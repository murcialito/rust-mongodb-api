FROM innovation-hub/base-images/javase-11-latest:latest
RUN yum install -y openssl-devel
COPY target/release/rust-api /usr/local/bin/rust-api
CMD ["rust-api"]