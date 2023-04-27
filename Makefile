mock:
	chmod +x ./mock_server/run.sh
	./mock_server/run.sh

run:
	cargo run --release -- --host http://localhost:3000 --report-file=report.html --no-reset-metrics