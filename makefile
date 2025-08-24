generate_specs:
	cd fixtures/typespec && \
	npx tsp format src/*.tsp && \
 	npx tsp format src/**/*.tsp && \
 	npx tsp compile src/one_route_basic_types