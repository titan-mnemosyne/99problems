#include <iostream>

template <class T>
T last(T *list, int size)
{
  if (size == 0)
  {
    return 0;
  }
  return list[size - 1];
}

int main()
{
  int nums[5] = {0, 1, 2, 3, 4};
  char letters[5] = {'a', 'b', 'c', 'd', 'e'};
  int empty[0];
  std::cout << last(nums, sizeof(nums) / sizeof(*nums)) << "\n";
  std::cout << last(letters, sizeof(letters) / sizeof(*letters)) << "\n";
  std::cout << last(empty, sizeof(empty) / sizeof(*empty)) << "\n";
}
