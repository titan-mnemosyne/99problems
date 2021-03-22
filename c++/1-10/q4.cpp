#include <iostream>
#include <vector>

using namespace std;

template <class T>
int len(vector<T> list)
{
  return list.size();
}

int main()
{
  vector<int> nums = {0, 5, 2, 1, 3, 3, 32, 2, 2, 1};
  vector<char> letters = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'};
  vector<bool> empty;
  cout << len(nums) << '\n';
  cout << len(letters) << '\n';
  cout << len(empty) << '\n';
}