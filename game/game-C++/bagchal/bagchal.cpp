#include <iostream>
#include <string>
#include <vector>
#include "constant.h"
#include "enum.h"
#include "utils.h"

using namespace std;

class Bagchal{
  public:
    int turn;
    int goat_counter;
    int goat_captured;
    int game_state;

    struct GameHistory{
      GameHistory(){}


      vector<vector<int>> board;
      int goat_count;
      int goat_captured;

      GameHistory(vector<vector<int>> board,int gc,int gcap){
        this->board = board;
        this->goat_count = gc;
        this->goat_captured = gcap;
      }

      // GameHistory(GameHistory& sample){
      //   goat_count = sample.goat_count;
      //   goat_captured = sample.goat_captured;
      //   board = sample.board;
      // }
    };
    vector<GameHistory> game_history;

    Bagchal(){};

    Bagchal(int turn,int g_counter,int g_captured,int game_state, GameHistory &history){

      this->turn = turn;
      this->goat_counter = g_counter;
      this->goat_captured = g_captured;
      this->game_state = game_state;
      this->game_history.push_back(GameHistory(history));
    }

    vector<vector<int>> board(){
      return game_history.back().board;
    }
    
    int move_count(){
      return (game_history.size() - 1);
    }

    Bagchal _new(){
      GameHistory history(DEFAULT_GAME_LAYOUT,0,0);
      Bagchal newGame(1,0,0,NOT_DECIDED,history);
      return newGame;
    }

    int char_to_cord(char ch){
      switch (ch)
      {
      case 'A':
        return 0;
      case 'B':
        return 1;
      case 'C':
        return 2;
      case 'D':
        return 3;
      case 'E':
        return 4;
      
      default:
        break;
      }
    }

    pair<vector<int>, vector<int>> pgn_unit_to_cord(vector<char> pgn){
      vector<int> source;
      vector<int> destination;
      if (slice(pgn,0,2) == "XX"){
        source.clear();
      }
      else{
        source = {5-int(pgn[1]), char_to_cord(pgn[0])};
      }
      destination = {5-int(pgn[3]), char_to_cord(pgn[2])};

      return {source,destination};
    }

    void clear_game(){
      GameHistory temp(DEFAULT_GAME_LAYOUT,0,0);
      this->goat_captured = 0;
      this->goat_counter = 0;
      this->game_state = NOT_DECIDED;
      this->turn = 1;
      game_history.clear();
      game_history.push_back(temp);
    }

    void load_game(string pgn){
      this->clear_game();
      vector<string> pgnList;
      vector<int> source;
      vector<int> destination;
      pgnList = split(pgn);

      for (auto i: pgnList){
        // auto ret = pgn_unit_to_cord(i);

      }
      
    }



};

int main(){
  Bagchal obj;
  obj.load_game("XX-00-A1-XA-01-00-AA");
}