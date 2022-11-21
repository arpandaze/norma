from libbaghchal import Baghchal
from copy import deepcopy


T_GOAT_CAPTURE = 4
T_GOT_TRAPPED = -2
T_TRAP_ESCAPE = 1
T_WIN = 10
T_LOSE = -10
T_DRAW = -6

# For Goat
G_GOAT_CAPTURED = -4
G_TIGER_TRAP = 2
G_TIGER_ESCAPE = -1
G_WIN = 10
G_LOSE = -10
G_DRAW = -6


class AlphaBeta:

  def __init__(self,depth=4):
    self.depth = depth

  def evaluation(self,board):

    total_gcaptured = board.goat_captured()
    total_tiger_trapped = board.trapped_tiger()
    isOver = board.game_status_check().decided

    if not isOver:
      return 150 * total_tiger_trapped - 120 * total_gcaptured

    winner = board.game_state()

    if winner == 1:
      return float('inf')
    
    elif winner == 2:
      return float('-inf')
    
    else:
      return 0

  def alpha_beta(self,board,depth=0,alpha=float('-inf'),beta=float('inf'),maximizing_player=True):

    if depth == 0 or board.game_status_check().decided:
      return self.evaluation(board), None

    possible_moves = board.get_possible_moves()
    best_move = None

    if maximizing_player:
      maxValue = float('-inf')
      best_move = None
      for i in possible_moves:
        board_cp = deepcopy(board)
        board_cp.make_move(i.move[0],i.move[1]) 

        result = self.alpha_beta(board_cp,depth-1,alpha,beta,False)

        if result[0] > maxValue:
          maxValue = result[0]
          best_move = i.move
          
        alpha = max(alpha,result[0])

        if beta <= alpha:
          break

      if best_move is None:
        return maxValue, None

      return maxValue, best_move

    else:
      minValue = float('inf')
      best_move = None
      for i in possible_moves:
        board_cp = deepcopy(board)
        board_cp.make_move(i.move[0],i.move[1])
        result = self.alpha_beta(board_cp,depth-1,alpha,beta,True)

        if result[0] < minValue:
          minValue = result[0]
          best_move = i.move

        beta = min(beta,result[0])

        if beta <= alpha:
          break

      if best_move is None:
        return minValue, None

      return minValue,best_move

  def best_bagh_move(self,board):
    assert not board.game_status_check().decided
    return self.alpha_beta(board,self.depth, maximizing_player=False) 
    
  def best_goat_move(self,board):
    assert not board.game_status_check().decided
    return self.alpha_beta(board,self.depth, maximizing_player=True)


a = AlphaBeta()
b = a.best_bagh_move(board=Baghchal.default())
print(b)







