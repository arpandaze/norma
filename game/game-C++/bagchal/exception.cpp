#include <iostream>
#include <string>
#include <sstream>
#include "exception.h"

using namespace std;


GameException::GameException(string msg){
  message = msg;
}

string GameException::getMessage(GameException obj){
  stringstream _msg;
  _msg << obj.message;
  return _msg.str();
}