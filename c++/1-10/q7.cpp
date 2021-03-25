#include <iostream>
#include <vector>
#include <typeinfo>

int main()
{
	std::vector<int> num;
	std::vector<char> letters;
	std::cout << typeid(num).name() << '\n';
	std::cout << typeid(letters).name() << '\n';
}
