build:
	g++ -Isrc -Iinclude src/main.cpp -o overhaul

release:
	g++ -Isrc -Iinclude src/main.cpp -overhaul -static-libgcc -static-libstdc++