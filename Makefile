mock:
	docker run --init -p 4010:4010 stoplight/prism:4 mock -h 0.0.0.0 https://discoveryprovider.audius.co/v1/swagger.json

run:
	cargo run --release -- --host http://0.0.0.0:4010 --report-file=report.html --no-reset-metrics
