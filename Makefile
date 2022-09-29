registry := my_registry
tag := latest


build:
	docker build -t rust-service .
	docker tag rust-service ${registry}:${tag}
	docker push ${registry}:${tag}

deploy:
	kubectl apply -f deployment.yaml
