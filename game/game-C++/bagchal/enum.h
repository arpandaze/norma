#ifndef ENUM_H
#define ENUM_H

enum GameState{
  NOT_DECIDED = 0,
  GOAT_WON = 1,
  TIGER_WON =2
};

extern GameState gameState;

#endif