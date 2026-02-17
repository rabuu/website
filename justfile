serve:
	python -m http.server ${PORT:-8000}

deploy:
	./scripts/deploy.sh
