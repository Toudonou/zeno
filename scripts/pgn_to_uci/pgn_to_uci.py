import io

import chess
import chess.pgn

moves = "1. e4 e5 2. Nc3 Nf6 3. Nf3 Nc6 4. Bc4 Nxe4 5. Nxe4 d5 { C55 Italian Game: Two Knights Defense } 6. Bxd5 Qxd5 7. Nc3 Qa5 8. a3 Bf5 9. d3 Bd6 10. O-O O-O 11. Be3 Qa6 12. b4 Ne7 13. Re1 Qc6 14. Bd2 Bg4 15. b5 Qb6 16. a4 Rfe8 17. a5 Qc5 18. Re4 Bxf3 19. Qxf3 a6 20. b6 Qc6 21. Rc4 Qxf3 22. gxf3 c6 23. Ne4 Rad8 24. Nxd6 Rxd6 25. Re1 Re6 26. f4 exf4 27. Rxe6 fxe6 28. Rxf4 Nd5 29. Rh4 h6 30. Kg2 Nf6 31. Bc3 e5 32. f4 exf4 33. Bxf6 gxf6 34. Rxh6 Kg7 35. Rh5 Re2+ 36. Kf3 Rxc2 37. Kxf4 Ra2 38. d4 Ra4 39. Ke4 Kg6 40. Rc5 Ra2 41. h4 Rh2 42. h5+ Rxh5 43. Rxh5 Kxh5 44. Kf5 Kh4 45. Kxf6 Kg4 46. Ke5 Kf3 47. Kd6 Ke4 48. Kc7 Kxd4 49. Kxb7 Kc3 50. Kxc6 Kb2 51. b7 Kc1 52. b8=Q Kd2 53. Kd5 Kc3 54. Qe5+ Kc2 55. Kc4 Kd2 56. Qf4+ Ke2 57. Kd4 Kd1 58. Kc4 Ke2 59. Kd4 Kd1 60. Kc4 Ke2 { The game is a draw. } 1/2-1/2"
pgn = io.StringIO(moves)
game = chess.pgn.read_game(pgn)
board = game.board()
for move in game.mainline_moves():
    board.push(move)
    print(move, end=" ")
