# SPRT results per features

## Static null move pruning
* Zeno current vs Zeno develop
```
--------------------------------------------------
Results of Zeno current vs Zeno Develop (8+0.08, NULL, NULL, UHO_Lichess_4852_v1.epd):
Elo: 72.32 +/- 6.98, nElo: 92.97 +/- 8.71
LOS: 100.00 %, DrawRatio: 36.10 %, PairsRatio: 2.52
Games: 6106, Wins: 2829, Losses: 1576, Draws: 1701, Points: 3679.5 (60.26 %)
Ptnml(0-2): [153, 402, 1102, 831, 565], WL/DD Ratio: 3.71
LLR: 4.61 (100.4%) (-4.60, 4.60) [70.00, 75.00]
--------------------------------------------------
SPRT ([70.00, 75.00]) completed - H1 was accepted
Finished match
Total Time: 02:23:00 (hours:minutes:seconds)
```

## SEE Pruning in the quiescence search
* Zeno current vs Zeno develop
```
--------------------------------------------------
Results of Zeno current vs Zeno Develop (8+0.08, NULL, NULL, UHO_Lichess_4852_v1.epd):
Elo: 20.63 +/- 5.96, nElo: 25.65 +/- 7.39
LOS: 100.00 %, DrawRatio: 37.95 %, PairsRatio: 1.25
Games: 8496, Wins: 3328, Losses: 2824, Draws: 2344, Points: 4500.0 (52.97 %)
Ptnml(0-2): [382, 787, 1612, 879, 588], WL/DD Ratio: 3.76
LLR: 4.61 (100.2%) (-4.60, 4.60) [10.00, 15.00]
--------------------------------------------------
SPRT ([10.00, 15.00]) completed - H1 was accepted
Finished match
Total Time: 03:16:28 (hours:minutes:seconds)
```

## Eval parameters tuning
* Zeno current vs Zeno 2.0
```
--------------------------------------------------
Results of Zeno Current vs Zeno 2.0 (8+0.08, NULL, NULL, UHO_Lichess_4852_v1.epd):
Elo: 28.83 +/- 5.96, nElo: 35.51 +/- 7.31
LOS: 100.00 %, DrawRatio: 37.29 %, PairsRatio: 1.43
Games: 8684, Wins: 3513, Losses: 2794, Draws: 2377, Points: 4701.5 (54.14 %)
Ptnml(0-2): [398, 722, 1619, 969, 634], WL/DD Ratio: 3.72
LLR: 4.60 (100.1%) (-4.60, 4.60) [20.00, 25.00]
--------------------------------------------------
SPRT ([20.00, 25.00]) completed - H1 was accepted
Finished match
Total Time: 03:36:23 (hours:minutes:seconds)
```

## Zeno 2.0 vs Zeno 1.0
* Zeno 2.0 vs Zeno 1.0
```
--------------------------------------------------
Results of Zeno 2.0 vs Zeno 1.0 (8+0.08, NULL, NULL, UHO_Lichess_4852_v1.epd):
Elo: 328.36 +/- 6.02, nElo: 449.43 +/- 4.82
LOS: 100.00 %, DrawRatio: 13.44 %, PairsRatio: 46.56
Games: 20000, Wins: 16205, Losses: 1454, Draws: 2341, Points: 17375.5 (86.88 %)
Ptnml(0-2): [38, 144, 1344, 1977, 6497], WL/DD Ratio: 11.22
--------------------------------------------------

Player: Zeno 1.0
  Timeouts: 0
  Crashed: 3

Finished match
Total Time: 06:02:16 (hours:minutes:seconds)

Post-Convergence rating estimation
done
   # PLAYER      : RATING    POINTS  PLAYED    (%)
   1 Zeno 2.0    : 2465.6   17375.5   20000   86.9%
   2 Zeno 1.0    : 2134.4    2624.5   20000   13.1%
```

## Late Move Reduction
* Zeno current vs Zeno develop
```
Results of Zeno current vs Zeno develop (8+0.08, NULL, NULL, UHO_Lichess_4852_v1.epd):
Elo: 21.24 +/- 4.95, nElo: 27.22 +/- 6.33
LOS: 100.00 %, DrawRatio: 40.51 %, PairsRatio: 1.34
Games: 11564, Wins: 4581, Losses: 3875, Draws: 3108, Points: 6135.0 (53.05 %)
Ptnml(0-2): [507, 964, 2342, 1254, 715], WL/DD Ratio: 4.26
LLR: 4.61 (100.3%) (-4.60, 4.60) [15.00, 20.00]
--------------------------------------------------
SPRT ([15.00, 20.00]) completed - H1 was accepted
Finished match
Total Time: 04:27:21 (hours:minutes:seconds)
```

## Time management and search refactoring
* Zeno current vs Zeno develop
```
Results of Zeno current vs Zeno develop (8+0.08, NULL, NULL, UHO_Lichess_4852_v1.epd):
Elo: 58.62 +/- 6.12, nElo: 71.16 +/- 7.29
LOS: 100.00 %, DrawRatio: 35.79 %, PairsRatio: 1.98
Games: 8724, Wins: 4000, Losses: 2542, Draws: 2182, Points: 5091.0 (58.36 %)
Ptnml(0-2): [314, 626, 1561, 1010, 851], WL/DD Ratio: 4.72
LLR: 4.60 (100.2%) (-4.60, 4.60) [55.00, 60.00]
--------------------------------------------------
SPRT ([55.00, 60.00]) completed - H1 was accepted
Finished match
Total Time: 03:03:53 (hours:minutes:seconds)
```


## Check Extensions
* Zeno current vs Zeno develop
```
Results of Zeno current vs Zeno develop (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 51.92 +/- 9.55, nElo: 64.48 +/- 11.68
LOS: 100.00 %, DrawRatio: 37.96 %, PairsRatio: 1.88
Games: 3398, Wins: 1516, Losses: 1012, Draws: 870, Points: 1951.0 (57.42 %)
Ptnml(0-2): [120, 246, 645, 386, 302], WL/DD Ratio: 4.42
LLR: 2.95 (100.0%) (-2.94, 2.94) [40.00, 45.00]
--------------------------------------------------
SPRT ([40.00, 45.00]) completed - H1 was accepted
Finished match
Total Time: 01:09:05 (hours:minutes:seconds)
```


## Change the conditions for returning an evaluation from the TT
* Zeno current vs Zeno develop
```
Results of Zeno current vs Zeno develop (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 12.05 +/- 7.02, nElo: 16.07 +/- 9.35
LOS: 99.96 %, DrawRatio: 37.87 %, PairsRatio: 1.18
Games: 5308, Wins: 1794, Losses: 1610, Draws: 1904, Points: 2746.0 (51.73 %)
Ptnml(0-2): [200, 557, 1005, 643, 249], WL/DD Ratio: 1.86
LLR: 2.97 (101.0%) (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
SPRT ([0.00, 5.00]) completed - H1 was accepted
Finished match
Total Time: 01:43:17 (hours:minutes:seconds)
```


## Search tables refactoring
* Zeno current vs Zeno develop
```
Results of Zeno current vs Zeno develop (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 21.14 +/- 9.71, nElo: 28.28 +/- 12.95
LOS: 100.00 %, DrawRatio: 38.21 %, PairsRatio: 1.25
Games: 2764, Wins: 951, Losses: 783, Draws: 1030, Points: 1466.0 (53.04 %)
Ptnml(0-2): [82, 297, 528, 321, 154], WL/DD Ratio: 1.56
LLR: 2.95 (100.0%) (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
SPRT ([0.00, 5.00]) completed - H1 was accepted

Player: Zeno current
  Timeouts: 1
  Crashed: 0

Finished match
Total Time: 00:54:56 (hours:minutes:seconds)
```

* Zeno current vs Zeno-1.0
```
Results of Zeno current vs Zeno 1.0 (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 161.30 +/- 9.38, nElo: 214.16 +/- 10.82
LOS: 100.00 %, DrawRatio: 26.53 %, PairsRatio: 7.55
Games: 3958, Wins: 2269, Losses: 553, Draws: 1136, Points: 2837.0 (71.68 %)
Ptnml(0-2): [31, 139, 525, 651, 633], WL/DD Ratio: 2.03
LLR: 2.95 (100.2%) (-2.94, 2.94) [138.00, 140.00]
--------------------------------------------------
SPRT ([138.00, 140.00]) completed - H1 was accepted
Finished match
Total Time: 01:09:17 (hours:minutes:seconds)
```


## Null Move Pruning
* Zeno current vs Zeno develop 
```
Results of Zeno current vs Zeno develop (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 141.48 +/- 13.88, nElo: 174.87 +/- 15.39
LOS: 100.00 %, DrawRatio: 27.78 %, PairsRatio: 5.09
Games: 1958, Wins: 1140, Losses: 384, Draws: 434, Points: 1357.0 (69.31 %)
Ptnml(0-2): [31, 85, 272, 279, 312], WL/DD Ratio: 6.77
LLR: 2.89 (100.0%) (-2.25, 2.89) [120.00, 125.00]
--------------------------------------------------
SPRT ([120.00, 125.00]) completed - H1 was accepted
Finished match
Total Time: 00:39:03 (hours:minutes:seconds)
```

* Zeno current vs Zeno-1.0
```
Results of Zeno current vs Zeno 1.0 (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 167.84 +/- 12.39, nElo: 208.55 +/- 13.22
LOS: 100.00 %, DrawRatio: 26.92 %, PairsRatio: 6.88
Games: 2652, Wins: 1641, Losses: 451, Draws: 560, Points: 1921.0 (72.44 %)
Ptnml(0-2): [30, 93, 357, 349, 497], WL/DD Ratio: 5.05
LLR: 2.89 (100.1%) (-2.25, 2.89) [135.00, 138.00]
--------------------------------------------------
SPRT ([135.00, 138.00]) completed - H1 was accepted
Finished match
Total Time: 00:51:59 (hours:minutes:seconds)
```


## History Heuristic
* Zeno current vs Zeno develop 
```
Results of Zeno current vs Zeno develop (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 7.12 +/- 8.57, nElo: 8.95 +/- 10.77
LOS: 94.84 %, DrawRatio: 43.70 %, PairsRatio: 1.11
Games: 4000, Wins: 1585, Losses: 1503, Draws: 912, Points: 2041.0 (51.02 %)
Ptnml(0-2): [224, 309, 874, 347, 246], WL/DD Ratio: 5.83
LLR: 0.53 (18.2%) (-2.25, 2.89) [0.00, 2.00]
--------------------------------------------------
Finished match
Total Time: 01:18:13 (hours:minutes:seconds)
```

* Zeno current vs Zeno-1.0
```
Results of Zeno current vs Zeno 1.0 (8+0.08, NULL, NULL, 8moves_v3.pgn):
Elo: 30.92 +/- 8.87, nElo: 37.74 +/- 10.77
LOS: 100.00 %, DrawRatio: 38.95 %, PairsRatio: 1.43
Games: 4000, Wins: 1692, Losses: 1337, Draws: 971, Points: 2177.5 (54.44 %)
Ptnml(0-2): [183, 319, 779, 398, 321], WL/DD Ratio: 5.13
LLR: 2.08 (72.1%) (-2.25, 2.89) [15.00, 18.00]
--------------------------------------------------

Player: Zeno 1.0
  Timeouts: 0
  Crashed: 1

Finished match
Total Time: 01:16:35 (hours:minutes:seconds)
```
