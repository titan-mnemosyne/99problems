#include <iostream>
#include <vector>

template <class T>
T removeDups(T *list, int size)
{
  T *out;
  T curr = NULL;
  int i = 0;
  int ind = 0;
  while (i < size)
  {
    if (list[i] != curr)
    {
      out[ind] = list[i];
      ind++;
    }
    else
  }
}
