from random import randint
from game import Game

import csv
from functools import reduce
#agent is the player that plays games and then, stores the data for training.

#use this to generate the training data, so generate the self play data, and the montecarlo search tree data. 
NO_OF_PLAY_DATA = 1000

class Agent:
    
    def __init__(self, value):
        self.noGames = 0
        self.epsilon = 10
        self.model = None
        self.value = value
    
    def getState(self, game: Game):
        #get the current state of the board
        return  game.game_history[-1]

    def moveState(self, currentState):
        #get the next board state from the current board state

        #use policy network to find the next move
        pass

    def move(self, game: Game, pgn):
        #function to Move the agent on the board
        currentState = self.getState(game)
        # source, target = self.moveState(currentState)
        count = 0
        if game.goat_counter < 20 and game.turn == 1:
            source = None
            target = [randint(0,4), randint(0,4)]
        else:
            source = [randint(0,4), randint(0,4)]
            target = [randint(0,4), randint(0,4)]

        while not game.check_move(source= source, target= target)["isValid"]:
            if game.goat_counter < 20 and game.turn == 1:
                source = None
                target = [randint(0,4), randint(0,4)]
            else:
                source = [randint(0,4), randint(0,4)]
                target = [randint(0,4), randint(0,4)]
            count += 1

            if count > 200:
                break
        
        if count <= 200:
            if source == None:
                pgn += "XX"
            else:
                match (source[1]):
                    case 0:
                        pgn += "A"
                    case 1:
                        pgn += "B"  
                    case 2:
                        pgn += "C"
                    case 3:
                        pgn += "D"
                    case 4:
                        pgn += "E" 

                pgn += str(5 - source[0])

            if target:
                match (target[1]):
                    case 0:
                        pgn += "A"
                    case 1:
                        pgn += "B"  
                    case 2:
                        pgn += "C"
                    case 3:
                        pgn += "D"
                    case 4:
                        pgn += "E" 

                pgn += str(5 - target[0])

                pgn += "-"

            game.move(source, target, ident_check=False)
            
        return count <= 200, game, pgn
    
    def saveLocalStates(self, state):
        #save all the states in a local memory
        self.moveMemory.append(state)
        pass

    def getSavedStates(self):
        # upon the end of the game return the local states. 
        pass



def recordData():
    count = 0
    playingGame = Game.new()
    # print(playingGame.game_history[-1])
    agentTiger = Agent(-1)
    agentGoat = Agent(1)
    pgn = ""

    # gameStatus = agentGoat.move(playingGame)
    # for i in range(NO_OF_PLAY_DATA):
        # agentGoat.getState()
        # agentTiger.moveState()
    while not playingGame.game_status_check()["decided"]:
        gameStatus, playingGame, pgn = agentGoat.move(playingGame, pgn)
        gameStatus, playingGame, pgn = agentTiger.move(playingGame, pgn)
        count += 1
        # print(playingGame.game_history)
        if count > 100:
            break
    
    #Generate the Data for Training:
     #[playerGoat, plaerTiger] [state for Tiger], [state for Goat] [1  or 0]

    # noOfStates = len(playingGame.game_history)
    # for k in range(noOfStates):
    #     print(playingGame.game_history[k])

    # with open("../data/valueNetworkTrain.csv", "w") as f:
    #     writer = csv.writer(f)
    #     header = ["Tiger", "Goat"]
    #     for i in range (1,6):
    #         for j in range (1,6):
    #             header.append(str(i) + "x" + str(j))
        
    #     header += ["Value Goat", "Value Tiger"]

    #     value = []
    #     for k in range(noOfStates):
    #         if k % 2 == 0:
    #             value += [0 , 1]
            
    #         flattenBoard = reduce(lambda z, y :z + y, playingGame.game_history[k])
    #         print(k)
    #         print(flattenBoard)
    #     writer.writerow()
    
    #     playingGame.game_history(i)
    # print(playingGame.game_status_check())
    # print(playingGame.game_state)
    # print(count)
    # print(len(playingGame.game_history))
    # print(pgn)

if __name__=="__main__":
    recordData()