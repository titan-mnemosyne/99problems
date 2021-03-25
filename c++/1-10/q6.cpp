#include <iostream>

template <class T>
bool isPalindrome(T *list, int size)
{
  int start = 0;
  int end = size - 1;
  while (end > start)
  {
    if (list[start] != list[end])
    {
      return false;
    }
    else
    {
      start++;
      end--;
    }
  }
  return true;
}

int main()
{
  int nums[5] = {0, 1, 2, 3, 4};
  char letters[5] = {'a', 'b', 'c', 'd', 'e'};
  int single[1] = {5};
  char palin[11] = {'m', 'a', 'd', 'a', 'm', 'i', 'm', 'a', 'd', 'a', 'm'};
  int even[4] = {2, 4, 4, 2};
  std::cout << isPalindrome(nums, 5) << '\n';
  std::cout << isPalindrome(letters, 5) << '\n';
  std::cout << isPalindrome(single, 1) << '\n';
  std::cout << isPalindrome(palin, 11) << '\n';
  std::cout << isPalindrome(even, 4) << '\n';
}
