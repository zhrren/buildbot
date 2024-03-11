FROM registry.cn-zhangjiakou.aliyuncs.com/publicci/rust:1.76-alpine-make

COPY . /workspace/src

WORKDIR /workspace/src
RUN cargo build --release

FROM registry.cn-zhangjiakou.aliyuncs.com/publicci/alpine:3.19.1

COPY --from=0 /workspace/src/target/release/buildbot /workspace/apps/

WORKDIR /workspace/apps
EXPOSE 10940
CMD ./buildbot
