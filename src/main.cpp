#include <iostream>

extern "C" void say_hello(const char *);

int main()
{
    std::string name {"World"};
    std::printf("Name: ");
    std::cin >> name;
    std::puts("");

    say_hello(name.c_str());
}
