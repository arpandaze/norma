#ifndef UTILS_H
#define UTILS_H

#include <vector>
#include <sstream>
#include <string>
#include <algorithm>

template<typename T>
std::string slice(std::vector<T> const &v, int m, int n)
{
    auto first = v.cbegin() + m;
    auto last = v.cbegin() + n + 1;
 
    std::vector<T> vec(first, last);
    std::stringstream vecToString;
    for (auto i: vec){
      vecToString << i;
    }


    return vecToString.str();
}


std::vector<std::string> split(std::string &str){
  str.erase(remove(str.begin(), str.end(),' '), str.end());
  std::vector<std::string> list(1);
  std::string rightToken;
  int vectorSize = 1;
  int pos;
  bool flag = true;
  if(str.find('-') == -1){
    list[vectorSize-1] = str;
    return list;
  }

  for(;;){
    pos = str.find('-');
    rightToken = str.substr(0,pos);
    str = str.substr(pos+1,str.length()-pos-1);
    list[vectorSize - 1] = rightToken;
    list.resize(vectorSize+1);
    vectorSize++;
    if(flag == false){
      break;
    }
  
    if(str.find('-') == -1){
      flag = false;
    }
  }
  list.resize(list.size()-1);
  return list;
}

#endif