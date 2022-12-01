from libbaghchal import Baghchal
from random import shuffle
INF = 1e6

class MM:

  def __init__(self,depth=4):
    self.depth = depth

  def evaluation(self,board, goat_eval = True):
    winner = board.game_state()

    if goat_eval:
      if winner == 1:
        return INF
      elif winner == 2:
        return -INF
      else:
        return 0
    else:
      if winner == 2:
        return INF
      elif winner == 1:
        return -INF
      else:
        return 0

  def goatMax(self,board: Baghchal,depth, maximizing_player=True):

    if depth == 0 or board.game_status_check().decided:
      print(f'depth: {depth}, player: {maximizing_player}, result: Evaluating')
      return self.evaluation(board, goat_eval=True), None

    if maximizing_player:
      maxValue = -INF
      best_move = None
      possible_moves = board.get_possible_moves()
      # shuffle(possible_moves)
      for i in possible_moves:
        board_cp = board.copy()
        board_cp.make_move(*i.move) 
        moveReward = board_cp.move_reward_goat()[-1]
    
        result = self.goatMax(board_cp,depth - 1, maximizing_player = False)
        
        
        reward = result[0] + moveReward

        print()
        print()
        print(f"Goat Move Current {i.move}.")
        print()
        print(f"Goat Move for Goat Maximizer at Depth {depth}.")
        print(f"moveReward: {moveReward}.")
        print(f"results: {result}.")
        print(f"total reward: {reward} and maxValue: {maxValue}.")
        if reward > maxValue:
          maxValue = reward
          best_move = i.move
        
        print(f"new maxValue: {maxValue}.")
        print()
        print()
      return maxValue, best_move

    else:
      minValue = INF
      best_move = None
      possible_moves = board.get_possible_moves()
      # shuffle(possible_moves)

      for i in possible_moves:
        board_cp = board.copy()
        board_cp.make_move(*i.move)
        moveReward = board_cp.move_reward_tiger()[-1]
        result = self.goatMax(board_cp,depth - 1, maximizing_player= True)

        reward = result[0] - moveReward

        print()
        print()
        print(f"Tiger Move Current {i.move}.")
        print()
        print(f"Tiger Move for Goat Maximizer at Depth {depth}.")
        print(f"moveReward: {moveReward}.")
        print(f"results: {result}.")
        print(f"total reward: {reward} and minValue: {minValue}.")
        if reward < minValue:
          minValue = reward
          best_move = i.move
        
        print(f"new minValue: {minValue}.")
        print()
        print()
      return minValue,best_move

  def tigerMax(self,board: Baghchal, depth, maximizing_player=True):

    if depth == 0 or board.game_status_check().decided:
      return self.evaluation(board, goat_eval=False), None

    if maximizing_player:
      maxValue = -INF
      best_move = None
      possible_moves = board.get_possible_moves()
      # shuffle(possible_moves)
      for i in possible_moves:
        board_cp = board.copy()
        board_cp.make_move(*i.move) 
        moveReward = board_cp.move_reward_tiger()[-1]
        
        result = self.tigerMax(board_cp,depth - 1, maximizing_player = False)
        
        reward = (1.5 - depth/5) * (result[0] + moveReward)
        
        print()
        print()
        print(f"Tiger Move Current {i.move}.")
        print()
        print(f"Tiger Move for Tiger Maximizer at Depth {depth}.")
        print(f"moveReward: {moveReward}.")
        print(f"results: {result}.")
        print(f"total reward: {reward} and maxValue: {maxValue}.")
        if reward > maxValue:

          maxValue = reward
          best_move = i.move
        
        print(f"new maxValue: {maxValue}.")
        print()
        print()
      return maxValue, best_move

    else:
      minValue = INF
      best_move = None
      possible_moves = board.get_possible_moves()
      # shuffle(possible_moves)

      for i in possible_moves:
        board_cp = board.copy()
        board_cp.make_move(*i.move)
        moveReward = board_cp.move_reward_goat()[-1]
        result = self.tigerMax(board_cp,depth - 1, maximizing_player= True)

        reward = (1.5 - depth/5) * (result[0] - moveReward)

        print()
        print()
        print(f"Goat Move Current {i.move}.")
        print()
        print(f"Goat Move for Tiger Maximizer at Depth {depth}.")
        print(f"moveReward: {moveReward}.")
        print(f"results: {result}.")
        print(f"total reward: {reward} and minValue: {minValue}.")
        if reward < minValue:
          minValue = reward
          best_move = i.move
        print(f"new minValue: {minValue}.")
        print()
        print()
      return minValue,best_move

  def best_bagh_move(self,board):
    assert not board.game_status_check().decided
    return self.tigerMax(board,depth=4, maximizing_player=True) 
    
  def best_goat_move(self,board):
    assert not board.game_status_check().decided
    return self.goatMax(board,self.depth, maximizing_player=True)

  def best_move(self,board:Baghchal):
    if board.turn() == 1:
      result = self.best_goat_move(board)
    else:
      result = self.best_bagh_move(board)

    print("The Result is: ", result)
    return result

# a = Minimax()
# board = Baghchal.default()

# while not board.game_status_check().decided:
#   res = a.best_move(board)
#   board.make_move(*res[1])

# print(board.pgn())

class AlphaBeta:

  def __init__(self,depth=6):
    self.depth = depth

  def evaluation(self,board,turn):
    
    total_gcaptured = board.goat_captured()
    total_tiger_trapped = board.trapped_tiger()
    isOver = board.game_status_check().decided

    if not isOver:
      if turn == 1:
        return 150 * total_tiger_trapped - 120 * total_gcaptured
      else:
        return 150 * total_gcaptured - 120 * total_tiger_trapped

    winner = board.game_state()

    if winner == 1:
      return float('inf')
    
    elif winner == 2:
      return float('-inf')
    
    else:
      return 0

  def alpha_beta(self,board,turn,depth=0,alpha=float('-inf'),beta=float('inf'),maximizing_player=True):
    if depth == 0 or board.game_status_check().decided:
      return self.evaluation(board,turn), None

    if maximizing_player:
      maxValue = float('-inf')
      best_move = None
      possible_moves = board.get_possible_moves()

      for i in possible_moves:
        board_cp = board.copy()
        board_cp.make_move(*i.move) 

        result = self.alpha_beta(board_cp,turn,depth-1,alpha,beta,False)

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
        result = self.alpha_beta(board_cp,turn,depth-1,alpha,beta,True)

        if result[0] < minValue:
          minValue = result[0]
          best_move = i.move

        beta = min(beta,result[0])

        if beta <= alpha:
          break

      return minValue,best_move

  def best_bagh_move(self,board,turn):
    assert not board.game_status_check().decided
    return self.alpha_beta(board,turn,self.depth, maximizing_player=True) 
    
  def best_goat_move(self,board,turn):
    assert not board.game_status_check().decided
    return self.alpha_beta(board,turn,self.depth, maximizing_player=True)

  def best_move(self,board:Baghchal):
    if board.turn() == 1:
      result = self.best_goat_move(board,turn=1)
    else:
      result = self.best_bagh_move(board,turn=-1)

    print(result)
    return result
