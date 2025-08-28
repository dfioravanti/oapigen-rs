TSP_DIRS := $(shell find fixtures/typespec/src -type f -name "main.tsp" -exec dirname {} \;)

compile-all: format-specs
	@echo "Starting compilation for all main.tsp files..."
	@echo "Found TSP projects in the following directories: $(TSP_DIRS)"
	@echo ""
	# Loop through each directory stored in the TSP_DIRS variable.
	@for dir in $(TSP_DIRS); do \
		echo "--- Compiling in $$dir ---"; \
		(npx tsp compile $$dir); \
		echo "--- Finished compiling in $$dir ---"; \
		echo ""; \
	done
	@echo "All compilations are complete."

format-specs:
	cd fixtures/typespec && \
	npx tsp format src/*.tsp && \
 	npx tsp format src/**/*.tsp 
