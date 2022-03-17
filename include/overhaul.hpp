#pragma once
#ifndef OVERHAUL_HPP
#define OVERHAUL_HPP

class Overhaul {
    const char* menu = 
R"([1] Add File
[2] Update File
[3] Update all files
[4] Show all files, links and locations.
[5] Exit
)";
    uint16_t ans; // must be above 16 bits or will result in undefined behaviour

public:
    Overhaul(){};
    
    void main_menu(){
        std::cout << menu << std::endl;
        std::cout << "Your Option: ";
        std::cin >> ans;
        switch (ans) {
            case 1: add_file(); break;
            case 2: /*Update Specific File Function*/ break;
            case 3: /*Update All Files Function*/ break;
            case 4: show_all(); break;
            case 5: std::cout << "! Thank you for using Overhaul !" << std::endl; exit(0); break;
            default: std::cout << "option: " << ans << "is not available" <<std::endl; main_menu(); break;
        }
        std::cout << "\n\n";
        main_menu();
    }

    void show_all(){
        system("cls");
        toml::table tbl = toml::parse_file("overhaul.toml");
        std::cout << "\n" << tbl << "\n";
    }
    
    void add_file(){
        system("cls");
        std::ofstream file("overhaul.toml", std::ios_base::app );

        std::string name, url, location;
        std::cout << "Name of file: ";
        std::cin >> name;
        std::cout << "Url: ";
        std::cin >> url;
        std::cout << "Location: ";
        std::cin >> location;

        file << "\n\n" << name << " = " << url;
        file << "\n" << name << "loc = " << location;

        
    }

    void update_file(){
        try{
            std::string url;
            toml::table tbl = toml::parse_file("overhaul.toml");
            std::optional<std::string> filename = tbl["parse_args"].value<std::string>();
            std::cout << *filename << std::endl;
        }
            catch (const toml::parse_error& err){
                std::cerr << "Parsing failed:\n" << err << "\n";
                exit(1);
        }
    }
    void update_all_files(){

    }

};
#endif