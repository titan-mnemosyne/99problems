#include <iostream>
#include <vector>

using namespace std;

template <class T>
vector<T> reverse(vector<T> list)
{
  vector<T> out;
  int i = list.size();
  while (i > 0)
  {
    out.push_back(list[i - 1]);
    i--;
  }
  return out;
}

int main()
{
  vector<int> num = {0, 1, 2, 3, 4};
  vector<char> letters = {'e', 'd', 'c', 'b', 'a'};
  vector<int> reverse_num = reverse(num);
  cout << '[' << reverse_num[0] << ',';
  cout << reverse_num[1] << ',' << reverse_num[2] << ',';
  cout << reverse_num[3] << ',' << reverse_num[4] << ']' << '\n';
  vector<char> reverse_let = reverse(letters);
  cout << '[' << reverse_let[0] << ',';
  cout << reverse_let[1] << ',' << reverse_let[2] << ',';
  cout << reverse_let[3] << ',' << reverse_let[4] << ']' << '\n';
}