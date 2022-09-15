#ifndef GAMEEXCEPTION_H
#define GAMEEXCEPTION_H

#include <string>

class GameException{
  public:
    std::string message;
    GameException(std::string msg);
    std::string getMessage(GameException obj);
};

#endif