#include <iostream>
#include <string>
#include <sstream>
#include "exception.h"

using namespace std;

class GameException{
  public:
    string message;
    GameException(string msg){
      message = msg;
    }

    string getMessage(GameException obj){
      stringstream _msg;
      _msg << obj.message;
      return _msg.str();
    }
};