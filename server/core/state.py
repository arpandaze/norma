import math
from game import Game
import copy
import uuid
import numpy as np
import collections

class HistoryNode:
    def __init__(self, game, state):
        self.children = []
        self.game = game
        self.state = state

    def addChild(self, child):
        self.children.append(child)

    def printState(self):
        count = 0
        visited, queue = [], collections.deque([self])
        visited.append(self)

        while queue:

            vertex = queue.popleft()
            count += 1
            print(vertex.state, end = ",")
            print(len(vertex.game.get_possible_state()))
            print(len(vertex.children))

            for child in vertex.children:
                if child not in visited:
                    visited.append(child)
                    queue.append(child)
        
        print()
        print()
        print(f"{count} states in a game of Baghchal")


if __name__ == "__main__":

    newGame = Game.new()
    GameHistory = HistoryNode(newGame, copy.deepcopy(newGame.game_history[-1]))

    grandParents = None
    parents = [GameHistory]
    childadded = True
    totalState = 0

    currentParent = parents[0]
    currentState = currentParent.game
    state = currentState.get_possible_state()
    totalState += len(state)

    for moves in state:
        newState = Game(game_id=str(uuid.uuid4()),tiger=currentState.tiger, goat=currentState.goat, goat_counter=currentState.goat_counter, goat_captured=currentState.goat_captured, game_state=currentState.game_state, game_history=copy.deepcopy(currentState.game_history), turn=currentState.turn, socket=currentState.socket)
        newState.move(moves[0], moves[1], ident_check=False)
        newGameHistory=HistoryNode(newState, copy.deepcopy(newState.game_history[-1]))
        currentParent.addChild(newGameHistory)

    grandParents = parents
    parents = currentParent.children

    while childadded: 
        childadded = False

        parentsList = []   

        for grandparent in grandParents:
            parents = grandparent.children
            print('Parent List', len(parents))
            parentsList += parents

            for parent in parents:
                currentParent = parent
                currentState = currentParent.game
                state = currentState.get_possible_state()
                print("States", parent.state, len(state))
                totalState += len(state)

                for moves in state:
                    newState = Game(game_id=str(uuid.uuid4()),tiger=currentState.tiger, goat=currentState.goat, goat_counter=currentState.goat_counter, goat_captured=currentState.goat_captured, game_state=currentState.game_state, game_history=copy.deepcopy(currentState.game_history), turn=currentState.turn, socket=currentState.socket)
                    newState.move(moves[0], moves[1], ident_check=False)
                    newGameHistory = HistoryNode(newState, copy.deepcopy(newState.game_history[-1]))
                    currentParent.addChild(newGameHistory)
                    childadded = True

            print("Total States", totalState)
            
        print('XXXX', len(parentsList))
        grandParents = parentsList

    
    GameHistory.printState()


    # newGame = Game.new()
    # currentState = [newGame]
    # GameHistory = HistoryNode(newGame, copy.deepcopy(newGame.game_history[-1]))
    # grandParents = None
    # parents= [GameHistory]
    # childadded = True

    # currentParent = parents[0]
    # currentState = currentParent.game
    # state = currentState.get_possible_state()

    # for moves in state:
    #     newState = Game(game_id=str(uuid.uuid4()),tiger=currentState.tiger, goat=currentState.goat, goat_counter=currentState.goat_counter, goat_captured=currentState.goat_captured, game_state=currentState.game_state, game_history=copy.deepcopy(currentState.game_history), turn=currentState.turn, socket=currentState.socket)
    #     newState.move(moves[0], moves[1], ident_check=False)
    #     newGameHistory = HistoryNode(newState, copy.deepcopy(newState.game_history[-1]))
    #     currentParent.addChild(newGameHistory)

    # grandParents = copy.deepcopy(parents)
    # parents = currentParent.children

    # while childadded:
    #     childadded = False

    #     grandparents = copy.deepcopy(parents)
    #     parentList = []
    #     for index, grandParent in enumerate(grandParents):
    #         parents = grandParent.children[index]
    #         parentList.append(parents)
            
    #         for parent in parents:
    #             currentParent = parent
    #             currentState = currentParent.game
    #             state = currentState.get_possible_state()

    #             for moves in state:
    #                 newState = Game(game_id=str(uuid.uuid4()),tiger=currentState.tiger, goat=currentState.goat, goat_counter=currentState.goat_counter, goat_captured=currentState.goat_captured, game_state=currentState.game_state, game_history=copy.deepcopy(currentState.game_history), turn=currentState.turn, socket=currentState.socket)
    #                 newState.move(moves[0], moves[1], ident_check=False)
    #                 newGameHistory = HistoryNode(newState, copy.deepcopy(newState.game_history[-1]))
    #                 currentParent.addChild(newGameHistory)
                
               
            




# __________ MAY BE NEEDED _______________

# from game import Game
    
# if __name__ == "__main__":
#     newGame = Game.new()
#     previousState = ["new"]
#     currentState = [newGame]
#     historyGame = {"new": currentState}
#     historyState = {"new": newGame.game_history[-1]}
#     decided = False

#         # for state in states:
#         #     gameOne = Game.new()
#         #     previousHistory = copy.deepcopy(gameOne.game_history[-1])
#         #     gameOne.move(state[0], state[1], ident_check = False)
#         #     if f"{previousHistory}" in historyGame:
#         #         historyGame[f"{previousHistory}"] += [gameOne]
#         #     else:
#         #         historyGame[f"{previousHistory}"] = [gameOne]

#     for currentStateVariable in previousState:
#         print(currentStateVariable)
#         currentState = historyGame[f"{currentStateVariable}"]
#         for state in currentState:
#             gameNew = Game(game_id=str(uuid.uuid4()),tiger=state.tiger, goat=state.goat, goat_counter=state.goat_counter, goat_captured=state.goat_captured, game_state=state.game_state, game_history=copy.deepcopy(state.game_history), turn=state.turn, socket=state.socket)
#             states = gameNew.get_possible_state()

#             for s in states:
#                 gameNew = Game(game_id=str(uuid.uuid4()),tiger=state.tiger, goat=state.goat, goat_counter=state.goat_counter, goat_captured=state.goat_captured, game_state=state.game_state, game_history=copy.deepcopy(state.game_history), turn=state.turn, socket=state.socket)
#                 previousHistory = copy.deepcopy(gameNew.game_history[-1])
                
#                 decided = gameNew.game_status_check()["decided"]
#                 if decided:
#                     break
                
#                 gameNew.move(s[0], s[1], ident_check = False)
#                 if f"{previousHistory}" in historyGame:
#                     historyGame[f"{previousHistory}"] += [gameNew]
#                     historyState[f"{previousHistory}"] += [gameNew.game_history[-1]]
#                 else:
#                     historyGame[f"{previousHistory}"] = [gameNew]
#                     historyState[f"{previousHistory}"] = [gameNew.game_history[-1]]

#     print(historyState)
#     print(currentStateVariable)
#     previousState = historyState[str(currentStateVariable)]
            
#     for currentStateVariable in previousState:
#         currentState = historyGame[f"{currentStateVariable}"]
#         print(currentState)
#         for state in currentState:
#             gameNew = Game(game_id=str(uuid.uuid4()),tiger=state.tiger, goat=state.goat, goat_counter=state.goat_counter, goat_captured=state.goat_captured, game_state=state.game_state, game_history=copy.deepcopy(state.game_history), turn=state.turn, socket=state.socket)
#             states = gameNew.get_possible_state()

#             for s in states:
#                 gameNew = Game(game_id=str(uuid.uuid4()),tiger=state.tiger, goat=state.goat, goat_counter=state.goat_counter, goat_captured=state.goat_captured, game_state=state.game_state, game_history=copy.deepcopy(state.game_history), turn=state.turn, socket=state.socket)
#                 previousHistory = copy.deepcopy(gameNew.game_history[-1])
                
#                 decided = gameNew.game_status_check()["decided"]
#                 if decided:
#                     break
                
#                 gameNew.move(s[0], s[1], ident_check = False)
#                 if f"{previousHistory}" in historyGame:
#                     historyGame[f"{previousHistory}"] += [gameNew]
#                     historyState[f"{previousHistory}"] += [gameNew.game_history[-1]]
#                 else:
#                     historyGame[f"{previousHistory}"] = [gameNew]
#                     historyState[f"{previousHistory}"] = [gameNew.game_history[-1]]
        
#         print(len(historyState[f'{historyState["new"]}']))