import { createContext, useState } from "react";
import { ItemTypes } from "../utils/config";
import { cloneDeep, set } from "lodash";

export const OnlineGameContext = createContext(null);

export const startingLayout = [
  [-1, 0, 0, 0, -1],
  [0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0],
  [-1, 0, 0, 0, -1],
];

// export const startingLayout = [
//   [1, -1, 1, 1, 1],
//   [1, 1, -1, 1, 1],
//   [1, 1, 1, -1, 1],
//   [1, 1, 1, 1, -1],
//   [0, 1, 1, 1, 0],
// ];
// export const startingLayout = [
//   [1, 1, 1, 1, 1],
//   [1, 1, 1, 1, 1],
//   [1, 1, 1, 1, 1],
//   [1, 1, 0, 0, 0],
//   [-1, -1, -1, 0, -1],
// ];

export const OnlineGameProvider = ({ children }) => {
  const [turn, setTurn] = useState(ItemTypes.GOAT);

  const [goatCounter, setGoatCounter] = useState(0);

  const [moveCounter, setMoveCounter] = useState(0);

  const [goatsCaptured, setGoatsCaptured] = useState(0);

  const [pgn, setPGN] = useState([]);

  const [gameResult, setGameResult] = useState({ decided: false });

  const [moveHistory, setMoveHistory] = useState([
    {
      game: startingLayout,
      goatCount: 0,
      goatsCaptured: 0,
    },
  ]);

  const [game, setGame] = useState(startingLayout);

  const nextMove = () => {
    let nextGameState = moveHistory[moveCounter + 1];
    setGame(nextGameState.game);
    setGoatCounter(nextGameState.goatCount);
    setGoatsCaptured(nextGameState.goatsCaptured);
    setMoveCounter((move) => move + 1);
  };

  const previousMove = () => {
    let prevGameState = moveHistory[moveCounter - 1];
    // console.log(prevGameState.game);
    setGame(prevGameState.game);
    setGoatCounter(prevGameState.goatCount);
    setGoatsCaptured(prevGameState.goatsCaptured);
    setMoveCounter((move) => move - 1);
  };

  const playFromThisPoint = () => {
    setMoveHistory(moveHistory.slice(0, moveCounter + 1));
    setPGN(pgn.slice(0, moveCounter));
    if (gameResult.decided == true) {
      setGameResult({ decided: false });
    }
  };

  const gameStatusCheck = (position, capturedThisRound) => {
    if (goatsCaptured + (capturedThisRound ? 1 : 0) >= 5) {
      return { decided: true, wonBy: ItemTypes.TIGER };
    }

    let tigerHasMove = (() => {
      for (let i = 0; i < 5; i++) {
        for (let j = 0; j < 5; j++) {
          if (position[i][j] === -1) {
            for (let k = -2; k <= 2; k++) {
              for (let l = -2; l <= 2; l++) {
                let thisPieceHasAMove = checkMove(
                  {
                    source: [i, j],
                    target: [i + k, j + l],
                  },
                  position,
                  ItemTypes.TIGER
                );
                if (thisPieceHasAMove["isValid"]) {
                  return true;
                }
              }
            }
          }
        }
      }
      return false;
    })();

    // console.log(tigerHasMove);
    // console.log(turn);

    let goatHasMove = (() => {
      for (let i = 0; i < 5; i++) {
        for (let j = 0; j < 5; j++) {
          if (position[i][j] === 1) {
            for (let k = -2; k <= 2; k++) {
              for (let l = -2; l <= 2; l++) {
                let thisPieceHasAMove = checkMove(
                  {
                    source: [i, j],
                    target: [i + k, j + l],
                  },
                  position,
                  ItemTypes.GOAT
                );
                if (thisPieceHasAMove["isValid"]) {
                  return true;
                }
              }
            }
          }
        }
      }
      return false;
    })();

    if (!goatHasMove) {
      if (goatCounter != 20) {
        goatHasMove = true;
      }
    }

    if (turn == ItemTypes.TIGER) {
      if (!goatHasMove) {
        return { decided: true, wonBy: ItemTypes.TIGER };
      }
    } else if (turn == ItemTypes.GOAT) {
      if (!tigerHasMove) {
        return { decided: true, wonBy: ItemTypes.GOAT };
      }
    }

    return { decided: false };
  };

  const cordToPGN = ({ source: [x, y], target: [m, n] }) => {
    let secondChar = x === "X" ? x : (5 - parseInt(x)).toString();
    let fourthChar = m === "X" ? m : (5 - parseInt(m)).toString();
    // let fourthChar = m.toString();

    let [firstChar, thirdChar] = [y, n].map((item) => {
      switch (parseInt(item)) {
        case 0:
          return "A";
        case 1:
          return "B";
        case 2:
          return "C";
        case 3:
          return "D";
        case 4:
          return "E";
        default:
          return "X";
      }
    });
    return firstChar + secondChar + thirdChar + fourthChar;
  };

  const makeMove = ({ source: [x, y], target: [m, n] }) => {
    let eval_move = checkMove({
      source: [parseInt(x), parseInt(y)],
      target: [parseInt(m), parseInt(n)],
    });

    if (eval_move["isValid"]) {
      let new_state = cloneDeep(game);

      let captured = eval_move["isCaptureMove"];
      if (captured) {
        new_state[eval_move["capturePiece"][0]][
          eval_move["capturePiece"][1]
        ] = 0;
        setGoatsCaptured((goatsCap) => goatsCap + 1);
      }

      setMoveHistory((hist) => {
        let new_history = cloneDeep(hist);

        new_history.push({
          game: new_state,
          goatCount: goatCounter,
          goatsCaptured: captured ? goatsCaptured + 1 : goatsCaptured,
        });

        return new_history;
      });

      new_state[m][n] = new_state[x][y];
      new_state[x][y] = 0;

      let gameStatusAfterMove = gameStatusCheck(
        new_state,
        eval_move["isCaptureMove"]
      );
      if (gameStatusAfterMove.decided === true) {
        setGameResult(gameStatusAfterMove);
      }

      setMoveCounter((moveCount) => moveCount + 1);

      setGame((_) => new_state);
      setPGN((_) => [...pgn, cordToPGN({ source: [x, y], target: [m, n] })]);
      //console.log(x, y, m, n);
    }
  };

  const placeGoat = ({ target: [m, n] }) => {
    if (game[m][n] !== 0 || gameResult.decided) {
      return false;
    } else {
      let new_state = [...game];

      let new_history = cloneDeep(moveHistory);

      if (moveCounter + 1 !== moveHistory.length) {
        const reason = "Cannot move while navigating history!";
        // console.log(`Move Counter: ${moveCounter}`);
        // console.log(`Move History: ${moveHistory.length}`);
        return { isValid: false, reason: reason };
      }

      new_history.push({
        game: game,
        goatCount: goatCounter + 1,
        goatsCaptured: goatsCaptured,
      });

      setMoveCounter((moveCount) => moveCount + 1);
      setMoveHistory((_) => new_history);

      new_state[m][n] = 1;
      // console.log("" + new_state);
      let gameStatusAfterMove = gameStatusCheck(new_state, false);
      if (gameStatusAfterMove.decided === true) {
        setGameResult(gameStatusAfterMove);
      }

      setGame(new_state);
      setGoatCounter((goatCount) => goatCount + 1);
      setPGN(() => [...pgn, cordToPGN({ source: ["X", "X"], target: [m, n] })]);
    }
  };

  const checkMove = (
    { source: [x, y], target: [m, n] },
    position = game,
    assumingTurn = turn
  ) => {
    x = parseInt(x);
    y = parseInt(y);
    m = parseInt(m);
    n = parseInt(n);
    // console.log(x + " " + y + " source" + m + " " + n + " destination");
    // console.log(game);
    if (x < 0 || y < 0 || m < 0 || n < 0 || x > 4 || y > 4 || m > 4 || n > 4) {
      const reason = "Cannot move outside the board!";
      // console.log(reason);
      return { isValid: false, reason: reason };
    }

    if (gameResult.decided) {
      const reason = "Cannot move after game has been decided!";
      // console.log(reason);
      return { isValid: false, reason: reason };
    }

    if (moveCounter + 1 !== moveHistory.length) {
      const reason = "Cannot move while navigating history!";
      // console.log(reason);
      // console.log(`Move Counter: ${moveCounter}`);
      // console.log(`Move History: ${moveHistory.length}`);
      return { isValid: false, reason: reason };
    }

    // if (
    //   !(
    //     (assumingTurn === ItemTypes.GOAT && position[x][y] === 1) ||
    //     (assumingTurn === ItemTypes.TIGER && position[x][y] === -1)
    //   )
    // ) {
    //   const reason = "Cannot move in other's turn!";
    //   return { isValid: false, reason: reason };
    // }

    // if (assumingTurn === ItemTypes.GOAT) {
    //   if (goatCounter < 20) {
    //     const reason = "Can't move goat before all goats are placed";
    //     console.log(reason);
    //     return { isValid: false, reason: reason };
    //   }
    // }

    if (position[m][n] !== 0) {
      const reason = "Target already has a piece!";
      // console.log(reason);
      return { isValid: false, reason: reason };
    }

    let xDiffAbs = Math.abs(x - m);
    let yDiffAbs = Math.abs(y - n);
    let xDiff = m - x;
    let yDiff = n - y;
    let sSum = x + y;
    let tSum = m + n;

    if (xDiffAbs === 0 && yDiffAbs === 0) {
      const reason = "Source and target can't be same!";
      // console.log(reason);
      return { isValid: false, reason: reason };
    }

    // Tiger can jump goats
    if (
      // (2,0), (2,2), (0, 2), (2, 2)
      (xDiffAbs === 2 && yDiffAbs === 0) ||
      (yDiffAbs === 2 && (xDiffAbs === 0 || xDiffAbs === 2))
    ) {
      if (xDiffAbs === 2 && yDiffAbs === 2) {
        if (sSum % 2 !== 0) {
          const reason = "Cannot jump diagonally from odd positions!";
          // console.log(reason);
          return { isValid: false, reason: reason };
        }
      }
      let pieceToCapture = [x + xDiff / 2, y + yDiff / 2];
      // console.log(pieceToCapture);
      //Check if piece to capture is goat
      if (position[pieceToCapture[0]][pieceToCapture[1]] === 1) {
        const reason = "Can capture goat!";
        // console.log(reason);
        return {
          isValid: true,
          isCaptureMove: true,
          capturePiece: pieceToCapture,
          reason: reason,
        };
      } else {
        const reason = "Cannot capture tiger!";
        // console.log(reason);
        return { isValid: false, reason: reason };
      }
    }

    // Can't move distance more than 2
    if (xDiffAbs > 1 || yDiffAbs > 1) {
      const reason = "Cannot move distance more than 2!";
      // console.log(reason);
      return { isValid: false, reason: reason };
    } // Can't move from odd position to another odd position
    // Example: 0,1 (0+1 = 1 odd) to 1,2 (1+2 = 3 odd)
    else if (sSum % 2) {
      if (tSum % 2) {
        const reason = "Can't move from odd position to another odd position!";
        // console.log(reason);
        return { isValid: false, reason: reason };
      }
    }

    const reason = "Default move!";
    return { isValid: true, isCaptureMove: false, reason: reason };
  };

  const value = {
    turn,
    setTurn,
    game,
    setGame,
    makeMove,
    placeGoat,
    checkMove,
    goatCounter,
    goatsCaptured,
    nextMove,
    previousMove,
    moveCounter,
    moveHistory,
    gameResult,
    setGoatCounter,
    setGameResult,
    setMoveHistory,
    startingLayout,
    setGoatsCaptured,
    playFromThisPoint,
    cordToPGN,
    setPGN,
    startingLayout,
    pgn,
    setPGN,
    setMoveCounter,
    gameStatusCheck,
  };

  return (
    <OnlineGameContext.Provider value={value}>
      {children}
    </OnlineGameContext.Provider>
  );
};
