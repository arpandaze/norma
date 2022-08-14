from game import Game
import copy
import uuid

class HistoryNode:
    def __init__(self, game, state):
        self.children = []
        self.game = game
        self.state = state

    def addChild(self, child):
        self.children.append(child)

    def printState(self):
        print(self.state)

        if len(self.children) >= 0:
            for child in self.children:
                child.printState()


if __name__ == "__main__":
    
    newGame = Game.new()
    previousState = ["X"]
    currentState = [newGame]
    GameHistory = HistoryNode(newGame, copy.deepcopy(newGame.game_history[-1]))
    
    parent = GameHistory
    newParent = [parent]
     
    # i = 0
    childadded = True

    while childadded:

        childadded = False

        for child in newParent:
            # child.printState()
            currentState = child.game
            state = currentState.get_possible_state()
            # print()
            # print()

            # print()
            # print()
            # print()
            # print()
            # print()
            # print()
            # print(i)
            # print(currentState.turn)
            for moves in state:
                newState = Game(game_id=str(uuid.uuid4()),tiger=currentState.tiger, goat=currentState.goat, goat_counter=currentState.goat_counter, goat_captured=currentState.goat_captured, game_state=currentState.game_state, game_history=copy.deepcopy(currentState.game_history), turn=currentState.turn, socket=currentState.socket)
                newState.move(moves[0], moves[1], ident_check=False)
                print(newState.turn)
                newGameHistory = HistoryNode(newState, copy.deepcopy(newState.game_history[-1]))
                child.addChild(newGameHistory)
                parent = child
                childadded = True

        
        newParent = parent.children
        
        # i += 1

    
        print()
        print()

        print()
        print()
        print()
        print()
        print()
        print()
        print()
        print()
        print(childadded)
        print()
        print()
        print()

        print()
        print()
        print()
        print()
        print()
        print()
        print()
        print()
        # # for child in newParent:
        # #     child.printState()
        # #     currentState = child.game
        # #     state = currentState.get_possible_state()
        # #     print(currentState.turn)
        # #     print(state)
        # #     for moves in state:
        # #         newState = Game(game_id=str(uuid.uuid4()),tiger=currentState.tiger, goat=currentState.goat, goat_counter=currentState.goat_counter, goat_captured=currentState.goat_captured, game_state=currentState.game_state, game_history=copy.deepcopy(currentState.game_history), turn=currentState.turn, socket=currentState.socket)
        # #         newState.move(moves[0], moves[1], ident_check=False)
        # #         newGameHistory = HistoryNode(newState, copy.deepcopy(currentState.game_history[-1]))
        # #         child.addChild(newGameHistory)

        
        # i += 1
        # print("HERRRRREUIEURIUEIRUIWUEIWUEIUWIEUIWUEIWUEIWUEIUWIEUIWUEIUEI")
        # if i >= 200:
        #     break
        

        
           
    GameHistory.printState()