from libbaghchal import Baghchal
from copy import deepcopy
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

    if maximizing_player:
      maxValue = float('-inf')
      best_move = None
      possible_moves = board.get_possible_moves()

      for i in possible_moves:
        board_cp = board.copy()
        board_cp.make_move(*i.move) 

        result = self.alpha_beta(board_cp,depth-1,alpha,beta,False)

        if result[0] > maxValue:
          maxValue = result[0]
          best_move = i.move
          
        alpha = max(alpha,result[0])

        if beta <= alpha:
          break

      return maxValue, best_move

    else:
      minValue = float('inf')
      best_move = None
      possible_moves = board.get_possible_moves()

      for i in possible_moves:
        board_cp = board.copy()
        board_cp.make_move(*i.move)
        result = self.alpha_beta(board_cp,depth-1,alpha,beta,True)

        if result[0] < minValue:
          minValue = result[0]
          best_move = i.move

        beta = min(beta,result[0])

        if beta <= alpha:
          break

      return minValue,best_move

  def best_bagh_move(self,board):
    assert not board.game_status_check().decided
    return self.alpha_beta(board,self.depth, maximizing_player=False) 
    
  def best_goat_move(self,board):
    assert not board.game_status_check().decided
    return self.alpha_beta(board,self.depth, maximizing_player=True)

  def best_move(self,board:Baghchal):
    if board.turn() == 1:
      result = self.best_goat_move(board)
    
    else:
      result = self.best_bagh_move(board)
    
    return result






