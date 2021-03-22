#include <iostream>

template <class T>
T k(T *list, int k, int size)
{
  if (k >= size)
  {
    return 0;
  }
  return *(list + k);
}

int main()
{
  int nums[5] = {0, 1, 2, 3, 4};
  char letters[5] = {'a', 'b', 'c', 'd', 'e'};
  int empty[0];
  std::cout << k(nums, 0, 5) << "\n";
  std::cout << k(nums, 3, 5) << "\n";
  std::cout << k(nums, 7, 5) << "\n";
  std::cout << k(letters, 2, 5) << "\n";
  std::cout << k(letters, 3, 5) << "\n";
  std::cout << k(empty, 0, 0) << "\n";
}