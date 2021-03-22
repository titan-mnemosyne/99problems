#include <iostream>
#include <vector>

using namespace std;

template <class T>
T nextToLast(std::vector<T> list)
{
  if (list.size() == 0)
  {
    return 0;
  }
  int ind = list.size() - 2;
  return list[ind];
}

int main()
{
  vector<int> nums = {0, 1, 2, 3, 4};
  vector<char> letters = {'a', 'b', 'c', 'd', 'e'};
  vector<int> empty;
  cout << nextToLast(nums) << "\n";
  cout << nextToLast(letters) << "\n";
  cout << nextToLast(empty) << "\n";
}