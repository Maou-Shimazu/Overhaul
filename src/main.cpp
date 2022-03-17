#include <iostream>
#include <toml++/toml.h> //comment this line out and use the one below.
#include <fstream>
#include <filesystem>

#ifndef INCLUDE_TOMLPLUSPLUS_H
#include "../include/toml++/toml.h"
#endif

#include "../include/overhaul.hpp"
Overhaul overhaul;

int main(int argc, char** argv){
    std::cout << "<------------------ Welcome to Overhaul. ------------------>" << std::endl;
    overhaul.main_menu();


    //-----------------------toml parser -------------------------------
   
   return 0;
}