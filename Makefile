build:
	g++ -Isrc -Iinclude src/main.cpp -o overhaul

run:
	g++ -Isrc -Iinclude src/main.cpp -o overhaul
	./overhaul

release:
	g++ -Isrc -Iinclude src/main.cpp -overhaul -static-libgcc -static-libstdc++
