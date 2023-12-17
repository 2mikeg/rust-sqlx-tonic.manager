new_mig:
	sqlx migrate add -r init

up_mig:
	source .env && sqlx migrate run

run:
	source .env && cargo run