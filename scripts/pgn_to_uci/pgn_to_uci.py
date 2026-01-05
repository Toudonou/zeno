import chess
import chess.pgn
import io

moves = "1. Nf3 d5 2. Nc3 e6 3. e4 { C00 French Defense: Two Knights Variation } d4 4. Nb5 Nc6 5. c3 dxc3 6. dxc3 e5 7. Qxd8+ Kxd8 8. Bc4 a6 9. Na3 f5 10. exf5 Bxf5 11. Nh4 Bd7 12. Bg5+ Be7 13. Nf3 e4 14. Bxe7+ Kxe7 15. Ng5 Nf6 16. Be2 h6 17. Nh3 Rhd8 18. Nf4 g5 19. Ng6+ Kf7 20. O-O Kxg6 21. Rad1 Be6 22. Rxd8 Rxd8 23. Bc4 Bf5 24. b4 Ne5 25. Bb3 Rd3 26. Nb1 Kg7 27. Bc2 Rd8 28. f4 gxf4 29. Rxf4 Be6 30. Rf1 Bxa2 31. Na3 Nd3 32. Ra1 Be6 33. Rd1 Nd5 34. Rd2 Nxc3 35. Nb1 Nb5 36. Re2 Bf5 37. Re3 Nxb4 38. Bb3 Nd4 39. Ba4 Nbc2 40. Rg3+ Kh7 41. Nc3 b5 42. Bxc2 Nxc2 43. h3 Rd2 44. Rg4 Bxg4 45. hxg4 e3 46. g5 b4 47. Ne2 Rxe2 48. Kh2 hxg5 49. Kg3 Rf2 50. Kg4 Rxg2+ 51. Kh3 Rd2 52. Kg4 Kg6 53. Kf3 b3 54. Kg4 e2 55. Kh3 e1=Q 56. Kg4 Rd8 57. Kh3 Rf8 58. Kh2 Ne3 59. Kh3 Ra8 60. Kh2 Rb8 61. Kh3 Ra8 "
pgn = io.StringIO(moves)
game = chess.pgn.read_game(pgn)
board = game.board()
for move in game.mainline_moves():
    board.push(move)
    print(move, end=' ')
