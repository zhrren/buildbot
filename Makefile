SHELL := /bin/bash
HIDE ?= @

.PHONY: build

name := "buildbot"
port := 10243

## ALIYUN_REGISTRY, ALIYUN_USERNAME, ALIYUN_REGISTRY_PWD,

clean:
	$(HIDE)cargo clean

fix:
	$(HIDE)cargo fmt

gen:
	$(HIDE)which sea-orm-cli >/dev/null || cargo install sea-orm-cli
	$(HIDE)sea-orm-cli generate entity -o src/domain/entity

build:
	$(HIDE)cargo build

test:
	$(HIDE)cargo test

docker-build:
	$(HIDE)docker build -f Dockerfile -t $(name) .

docker-push: env := "dev"
docker-push: version := $(shell date +%y%m%d%H%M%S)
docker-push: tag := $(name):$(env)-$(version)
docker-push:
	$(HIDE)echo $(tag)
	$(HIDE)docker build -f Dockerfile -t $(tag) .
	$(HIDE)echo "$(ALIYUN_REGISTRY_PWD)" | docker login $(ALIYUN_REGISTRY) --username $(ALIYUN_USERNAME) --password-stdin
	$(HIDE)docker tag $(tag) $(ALIYUN_REGISTRY)/weworkci/$(tag)
	$(HIDE)docker push $(ALIYUN_REGISTRY)/weworkci/$(tag)
	$(HIDE)echo $(ALIYUN_REGISTRY)/weworkci/$(tag)

docker-up: image := "registry.cn-zhangjiakou.aliyuncs.com/weworkci/buildbot:dev-231230210046"
docker-up:
	$(HIDE)docker run -it --rm --name $(name) --env-file .env --publish $(port):$(port) $(image) sh

docker-down:
	$(HIDE)docker rm -f $(name)
