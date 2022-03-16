#include <iostream>
#include <toml++/toml.h>

#ifndef INCLUDE_TOMLPLUSPLUS_H
#include "../include/toml++/toml.h"
#endif

int main(int argc, char** argv)
{
   try
   {
       toml::table tbl = toml::parse_file("config/url.toml");
       //std::cout << tbl << "\n";
       std::optional<std::string> parse_args = tbl["parse_args"].value<std::string>();
       std::cout << *parse_args << std::endl;
   }
   catch (const toml::parse_error& err)
   {
       std::cerr << "Parsing failed:\n" << err << "\n";
       return 1;
   }

   return 0;
}