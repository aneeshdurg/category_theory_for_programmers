all: run

challenges/target/debug/challenges:
	cd challenges && cargo build

run: challenges/target/debug/challenges
	./$^

clean:
	rm -rf challenges/target/
