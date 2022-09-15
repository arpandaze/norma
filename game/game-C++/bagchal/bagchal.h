#ifndef BAGCHAL_H
#define BAGCHAL_H

class Bagchal{
  public:
    int turn;
    int goat_counter;
    int goat_captured;
    int game_state;
    
    Bagchal(int turn,int g_counter,int g_captured,int game_state){};
};

#endif