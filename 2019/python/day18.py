lily_input = '''\
#################################################################################
#...........#.......................#...#.........#.....#.....#...Q.............#
###.#######.#################.#.###.###G#.#####.#.#.#.###.#.#.#.#######.#####.###
#...#.......................#.#.#.......#...#...#z#.#...#.#.#.#...#u..#....b#...#
#.#########################.###.#######.###.#.#####.###.#.#.#####.#.#.#####.###.#
#.............#...#l....#.#...#.....#.#.#...#...#...#...#.#.....#.#.#...#...#...#
#.###########.#.###.#.#.#.###.#####.#.#.#######.#.###.#N#.###.###.#.###.#####.#.#
#.#.........#.#.....#.#.....#.#.....#...#.......#.#.#.#.#...#...#.#...#.......#.#
#.#.#######.#.#.#####.#####.#.#.#####.###.#####.#.#.#.#####.###.#.###.###########
#.#.#.D.#...#.#.....#.....#.#.#.....#...#.#.....#...#...#...#...#...#...#.......#
#.#S#.###.###.#####.#####.#.#.#####.###.#.#.#######.###.#.###.#.###.###.#.#####.#
#...#.......#.#...#.#...#.#.#.#...#.#.#.#.#.#.......#.#.....#.#.#.....#...#.#.A.#
#.###########.#.#.#.#.###.#.#.#C#.#.#.#.#.#.#.#######.#######.###.#######.#.#.#.#
#.#.........#.#.#.#.#.#...#.#...#...#.#.#.#.#...#.#.....#.#...#...#...Y.#...#.#.#
#.#.#######.#.#.###.#.#.#############.#.#.#####.#.#.#.#.#.#.###.###.###.###.#.###
#.#.#...#.#...#...#...#.........#.....#.#.....#.#...#.#.#.#.......#.#.#.#...#...#
###.#.#.#.#######.###.#####.###.#.#.###.#.#.#.#.#####.#.#.#########.#.#.#######.#
#...#.#...#.......#.....#...#.#.#.#.#...#.#.#.#.#.....#...#...#m..#...#.#.....#w#
#.###.###.#.#.#####.#####.###.#.#.#.#.###.#.#.#.#.#.#######.#.#.#.###.#.#.###.#.#
#e..#...#...#.#.......#...#.....#.#.#...#.#.#.#...#.#...F...#...#.#...#.#...#...#
#.#.###.#######.#######.#.#####.###.###.#.#.#.#####.#.###########.#.###.#.#.###.#
#.#...#...#.......#...#.#.#...#.......#.#.#.#.#.....#...#i..#.I.#.#.#y#.#.#.#.#.#
#.###.#.#.#.#######.###.###.#.#######.#.#.#.###.#######.#.#.###.#M#.#.#.###.#.#.#
#.#.#.#.#.......#...#.#.....#...#.....#.#.#...#...#.#...#.#.....#.....#x#...#.#.#
#.#.#.#.#########.###.#.#######.###.###.#.###.###.#.#.###.#############.#.###.#.#
#.#.#.#.#.#.....#.#...#...#...#...#...#.#v..#...#...#f..#.#...#..o#.....#.#...#.#
#.#.#.#.#.#.###.#.#.#.###.#.#####.#####.#.###.#.###.###J#.#.#.#.#.#.#####.###.#.#
#...#.#...#.#.#...#.#...#.#.#...#.#...#.#.#...#.#...#...#.#.#...#.#.#...#...#...#
###.#.###.#.#.#####.#####.#.#.#.#.#.#.#.#.#.#####.###.###.#.#####.#.#.###.#.#.###
#...#...#.#...#...#.........#.#...#.#.#.#.#.#.....#...#.#.#.#...#.X.#.....#.#...#
#.#####.#####.#.#.#.#########.#####.#.#.#.#.#.#####.###.#.#O#.#.###########.###.#
#.#...#.....#...#.#.....#...#...#...#.#.#.#...#...#.#..k#...#.#.....#.......#.#.#
#.#B#######.#####.###.###.#.###.#.###.#.#.#####.#.#.#.###########.#.#.#######.#.#
#.#.......#.....#.#...#...#...#...#...#.#...#.#.#..j#.....#.......#.#.#.......#.#
#.#.#####.###.#.#.#####.#####.#.###.#.#.###.#.#.#######.#.#.#######.#.#######.#.#
#.#...#.....#.#.#.......#.....#.#.#.#.#.#.#.#.#.#...#...#.#...#...#.#.....#...#.#
#.#####.###.#.###########.#####.#.#.###.#.#.#.#K#.#.#.###R#.#T###.#.#####.#.###.#
#.#...#...#.#.....#.....#.#...#.#.#...#.#.#.#.#.#.#...#...#.#.#...#...#...#...#.#
#.#.#.###.#.#####.#.#.###.#.#.#.#.###.#.#.#.#.#.#.#####.#####.#.#####.#.###.#.#.#
#...#.....#.....#...#.....#.#.......#.........#...#...........#.......H.#...#..p#
#######################################.@.#######################################
#...........#.....#.....#...#...#.....#.........#...#.......#...#...#...........#
#.###.#####.###.#.#.#.#.#.#.#.#.#.#.#.#.#.#.###.#.#.#.#.###.#.#.#.#.###.#######.#
#.#.#.#...#.....#.#.#.#.#.#.#.#...#.#...#.#.#...#.#...#...#...#...#...#.#.......#
#.#.#.#.#########.###.#.#.#.#####.#.###.#.#.#.###.#######.###########.#.#.#######
#.#.#.#.........#...#.#.#.#.....#.#...#.#.#.#...#.......#...#.#...#...#.#.#.....#
#W#.#.#####.###.###.#.#.#.#####.#####.#.###.###.#######.###.#.#.#.#.###.#.#.###.#
#...#.#...#...#...#.#.#.#.#.....#.....#.#...#...#........g#.#...#.#.#..s#.#...#.#
###.#.#.#.###.###.#.#.#.#.#.#####.#####.#.#######.#######.#.#####.#.#.#####.###.#
#...#.#.#.#t..#...#.#.#.#.#...#...#...#.#.#.......#...#...#.....#.#.#.......#...#
#.###.#.#.###.#.###.#.#.#.###.###.#.#.#.#.#.#######.#.#########.#.#.#.#######.#.#
#...#...#...#.#.....#.#.#...#...#.#.#...#...#.......#...#...#...#...#...#.....#.#
###.#######.#######.#.#.###.###.#.#.#####.###.#########.#.#.#.###.#######.#####.#
#.#.#.....#...#...#...#.#r#.#.#...#.....#.#.#...#.....#...#.#.#.#...#.....#.....#
#.#.#.#######.#.#.#####.#.#.#.#########V#.#.###.#.###.#####.#.#.###.#.#####.#####
#...#.#.....#.#.#.#...#.#.#.#.........#.#...#.#.#...#.#...#...#...#...#.#...#...#
#P###.#.###.#.#.#.#.#.#.#.#.#.#####.###.###.#.#.###.#.###.#######.#####.#.#####.#
#.....#.#.....#.#.#.#...#.#.#.....#.....#.#...#...#.#...#.#.............#.......#
###.###.#####.#.#.###.###.#.#####.#######.###.###.#.###.#.#.#.#########.#######.#
#...#...#...#.#.#...#.....#.#.#...#.....#.#.....#.#.#.....#.#.........#.....#...#
#####.###.#.###.###.#######.#.#.###.###.#.#.#####.#.#######.#######.#.#####.#.###
#.....#...#.......#.#.#.....#.#.#...#...#...#...#.#.......#.#.#...#.#.....#.#.#.#
#.###.#.###########.#.#.#####.#.###.#.#######.#.#.#.#####.#.#.#.#.#.#####.###.#.#
#...#.#.#.....#.....#...#.....#...#.#...#...#.#...#...#...#.#...#.#.#...#...#.#d#
#.#.#.#.#.###.#.#########.#.#####.#.#####.#.#.#########.#.#.#####.#.#.#.###.#.#.#
#.#.#.#...#.#.#...#.......#.#...#.#.#...#.#.#.........#.#.#.....#.#...#...#.#.#.#
#.#.#######.#.###.#.#######.#.#.#.#.#.#.#.#.#########.#.#######.#.#########.#.#.#
#.#.........#...#.#...#.L.#...#c#.#...#.#.#...........#....q..#.#.........#.....#
#.#######.#####.#.###.###.#####.#.#####.#.#############.#####.#E#########.#######
#.#.....#.......#...#.....#...#.#.#...#.#.#...#...#.....#...#.#.#.......#.......#
#.#.###############.#####.###.#.#Z#.#.#.#.#.#.#.#.#.#####.###.#.#.#############.#
#.#.......#.....#.#.....#.....#...#.#...#...#.#.#...#...#.U.#...#.....#.........#
#.#.###.#.###.#.#.###.#######.#####.###.#.###.#.#####.#.###.#########.#.#########
#.#...#.#.....#.....#.......#.#.....#.#.#.#...#...#...#.#.........#...#.#.#.....#
#.###.#.###########.#######.#.#.#.###.#.#.#.#####.###.#.#.#######.#.#.#.#.#.#.#.#
#.#...#a#.........#...#.#...#.#.#.#...#.#.#.....#.....#...#.....#...#.#.#...#.#.#
#.#####.#.#####.#####.#.#.###.###.#.#.#.#.#####.#########.#.###.#######.#####.#.#
#.#...#.#.#...#.....#...#..n..#...#.#.#.#...#.#.........#.#.#...#.....#.#...#.#.#
#.#.#.#.#.#.#.#####.###.#######.###.#.#.###.#.#########.###.#.###.###.#.#.#.#.#.#
#...#...#...#....h#.............#...#...#.............#.....#.......#.....#...#.#
#################################################################################
'''

kevan_input = '''\
#################################################################################
#.............#.#..g..........#t........#...#...................#.....#.....#...#
#W#########.#.#.#.#########.#.#.#####.###.#.#############.#####.###.#.#.#.#.###.#
#.#...#.....#...#.#...#...#.#.#.#...#...#.#.#....s..#..o#...#.#...#.#...#.#.#...#
#.###.#.#######.#.###.#.#.#.###.#.#.#.#.#.#.#.#####.#.#.#.#.#.###.###.###.#.#.###
#.....#.#.....#.#.#...#.#.#.....#.#.#.#.#.#...#...#.#.#.#.#.#...#...#...#.#.#...#
###.###.#.#.###.#.#.###.#.#######.#I###.#.#####.#.#.#.#.###.###.###.#.###.#.###.#
#.#.#...#.#.....#.#.#...#.....#...#.....#n..#...#.#...#...#.#...#.#.#.#...#...#.#
#.#.#.#####.#####.#.#.#####.###.###########.#.###########.#.#.#.#.#X###.#####.#.#
#.#.#x..Y.#.....#.#...#.....#...#.......#...#.#.............#.#...#.....#...#...#
#.#.#####.#######.#.###.#.###.###.#####.#.###.#.#############.###########.#.###.#
#...#...#.....#...#...#.#.#.E.#......j#.#.#...#.#.#.K.......#.#.....#.....#...#.#
#.###.#.#####.#.#######.###.#.#.#######.#.###.#.#.#.#######.#.#.#.#.#.###.#####.#
#.#...#.#.....#.#.....#.#...#.#.#.#.....#...#.....#.#b..#...#...#.#...#.#.#.....#
#.#.#.###.###.#.#.###.#.#.#####V#.#.#####.#.#######.#.#.#.###.###.#####.#Z#.#####
#.#.#.#...#.#.#...#...#.#.........#.....#.#...........#.#...#.#.#...#...#.#...#.#
#.#.###.###.#.#.#####.#.#########.#####.#.#################.#.#.###.###.#.###.#.#
#...#...#.....#.#...#.....#..q#.......#.#.......#...#.......#.#.#...#...#...#...#
#.###.###.#####.#.#.#######.#.#########.#########.#.#.#######.#.#.###.#####.###.#
#.#...#...#.....#.#.F.#.....#.........#.#.........#.#.#.........#...#.H...#...#.#
###.#######.#####.###.#.#############U#.#.###.#####.#.#########.###.#.###.#.###.#
#...#.....#.#...#...#.#.#...#.......#...#.#...#...#.#.....#...#...#.#...#...#...#
#.###.###.#.###.###.###.###.#.###.#.#####.#####.#.#.#####.#.#######.#########.###
#.....#...#.....#...#.....#...#...#.....#.....#.#.#.......#.#.......#.....#...#.#
#.#####.#######.#.###.###.#.###.#######.#.###.#.#.#######.#.#.###.###.###J#.###.#
#.....#.....#...#...#.#.#.#...#.....#...#...#...#.#.#.....#.#...#.#...#...#.#...#
#####.#####.#.#####.#.#.#.###.#####.#.###.#####.#.#.#.#####.#.#.#.#.###.#.#.###.#
#.....#...#.#.#.....#...#.#.#.#...#.#.#.#.#...#.#.#.....#...#.#.#.#..u#.#.#.....#
#.#######.#.#.#.#######.#.#.#.#.#.#.#.#.###.#.###.#######.###.#.###.###.#.#####.#
#.#.......#...#h#.......#.#...#.#.#.#.#c#...#...#...#.L.#...#.#...#.#...#.#.....#
#.###.#.#######.#.#######.#####.#.#.#.#.#.#####.###.#.#.###.#.###.#.#.#####.#####
#...#.#.....#...#...#...P.#.....#...#.#.#.#.........#.#.....#...#...#.....#.#...#
###.#####.#.#.#######.#####.#########.#.#.#.#########.#########.#########.#.#.#.#
#.#.....#.#.#.....#...#.....#.#.....#.#.#.#.#.....#...#...#...#.#..p..#.#.#...#.#
#.#####.#.#.#####.#.#####.###.#.#.###.#.#.###.###.#.###.###.#.#Q#.###.#.#.#####.#
#.......#.#.....#..z#.....#.#...#.....#.#...#...#.#.#...#...#.#...#.#.#.#..v..#.#
#.#######.#.###.#####.#.###.#.#########.###.###.#.#.#.#.#.###.#####.#.#.###.###.#
#.#.......#...#.#.....#.#.#...#...#...#.#...#...#.#.#.#.#.#.#.........#...#...#.#
#.###########.#.#######.#.#.###.#.#.#.#.#.###.###.#.#.#.#.#.#############.###.#.#
#.............#.........#.......#...#...........#...#.#....................f#...#
#######################################.@.#######################################
#.........#.....#.........#.....#.............#...............#.........#.......#
#.#####.###.###.#.#####.###.###.#.#.###.#.#.###.#########.###.#.#####.#.###.###.#
#.#.....#...#...#.....#...#.#.#...#...#.#.#.....#.......#.#...#.#.....#...#...#.#
###.#####.###.#.#####.###.#.#.#######.#.#.#######.###.###.###.#.#.#.#####.###.#.#
#...#.....#...#...#.#.#.#.#...#.....#.#.#.#...#...#.#...#...#.#.#.#.#...#.....#.#
#.###.#####.#####.#.#.#.#.###.#####.#.###.###.#.###.###.###.###.#.#.#.#.#.#######
#.#...#...#.....#.#.#.#.#...#.....#.#...#.#...#...#...#...#.....#.#.#.#.#.#.....#
#.#.###.#.#####.#.#.#.#.###.#####.#.###.#.#.#####.#.#.#.#.#######.###.#.###.###.#
#.#.#...#...#...#...#.#...#.....#.....#.#.#.........#.#.#.....#.#..d..#...#.#.#.#
#.#.#.#.###.#.#######.#.#.###.###.#####.#.#.#########.#.#####.#.#########.#.#A#.#
#...#.#.#.#.#...#.....#.#...#.#...#.....#.#.#...#.....#.#.#..m#.......#...#.#.#r#
#.#####.#.#.###.#.#######.#.#.#.###.###.#.###.#C#######.#.#.#########.#.###.#.#.#
#...#...#.#...#...#.......#.#...#l..#.#.#.....#.........#...#.........#.......#.#
###.#.###.###.#####.#####.#####.#.###.#.#################.###.###############.#.#
#.#.#.#.............#.....#...#.#...#...#.......#.........#...#.......#...#...#.#
#.#.#.###################.#.#.#####.#.###.#####.#.#########.###.#####.#.#.#####.#
#...#...#a..#...........#.#.#.......#...#.....#.#.....#.....#...#.......#.......#
#.#####.#.#.#.#########.#.#.#.###########.###.#.#####.###.#.#.###.#############.#
#...#...#.#.D.#...#...#.#.#.#.#.........#.#...#.#...#...#.#.#...#.....#.....#...#
###.#.#.#.###.#.#.#.#.#M###.###.#######.###.###.#.#.###.#.#####.###.###.###.#.###
#.....#.#.#...#.#...#.#...#.#...#.....#.#...#...#.#.......#..e#...#.#...#.#.#...#
#######.#.###.#.#########.#.#.###.###.#.#.#######.#########.#####.#.#.###.#.#####
#.....#.#...#.#.......#.....#.#...#...#.#.........#...#.....#...#.#.#.#...#.....#
#.###.#####.#########.#.#####.#####.#.#.#.#########.#.#.#####.#.#.###.#.#.#####.#
#.#.#.#...#.........#.#.#.....#.....#.#.#.#...#.....#.#i#.....#...#...#.#...#...#
#.#.#.#.#.###.#####.#.###.#####.#####.#.#.#.###.###.#.#.#.#########.#######.#.#.#
#.#.#...#.#...#.#...#.....#...#...#...#.#.#.#...#...#.#.#.............#.....#.#.#
#.#.#####.#.###.#.#########.###.#.#####.#.#.#.###.#####.#.###########.#.#####.###
#.#.....#.#.#.....#.......#.#...#...#...#.#.....#...#...#.#......w..#...#...#...#
#.#.#.###R#.#.#########.#.#.#.#####.#.###.#########.#.#####.#######.#####.#.###.#
#.#.#.......#.#.......#.#...#.#...#...#.#...........#...#...#.....#.......#...#.#
#.###########.#.#.###.#.#.###.#.#.#####.###.###########T#.#######.###########.#.#
#.#.....#.....#.#.#.#...#.#...#.#...#...#...#...........#.#.....#.........#.#...#
#.#.###.###.#####.#.#####.#.#####.#.#.#.#.###.###########.#.#.#.#.#####.#.#.#####
#.#.#.#...#...#...#.......#.....#.#...#.#.#.#.#...........#.#.#.#...#...#...#...#
#.#.#.###.###.#.#.#############.#####.#.#.#.#.###.###########.#.#####.#####.###.#
#.#.#...#.#.....#.#.S.........#.#...#.#y#..k#.G...#.........B.#.....#.....#...#.#
#.#.#.#.#.#########.#########.#.#.#.###.###.###########.###########.#####.###.#.#
#.....#.#...................#.O...#.....#...............#...........N.....#.....#
#################################################################################
'''

input_0 = '''\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######
'''

input_1 = '''\
###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############
'''

input_2 = '''\
#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############
'''

input_3 = '''\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############
'''

import string

from collections import deque
from enum import Enum
from typing import List, Dict, Tuple, Set, Deque, Iterator

def main():
    for input_str in input_0, input_1, input_2, input_3, kevan_input:
        m = Map(input_str)
        print(m)
        print(list(m.reachable_keys()))
        print(m.best_route())

class Tile(Enum):
    wall = 0
    floor = 1
    key = 2
    door = 3

    @classmethod
    def new(cls, c: str): # -> Tile:
        if c == '#':
            return Tile.wall
        elif c == '.':
            return Tile.floor
        elif c.islower():
            return Tile.key
        elif c.isupper():
            return Tile.door
        else:
            assert False

    def __str__(self) -> str:
        if self == Tile.wall:
            return '#'
        elif self == Tile.floor:
            return '.'
        elif self == Tile.key:
            return '$'
        elif self == Tile.door:
            return '>'
        else:
            assert False

Position = Tuple[int, int]

class Map:
    def __init__(self, input_str: str):
        self.grid: List[List[Tile]] = []
        self.keys: Dict[Position, str] = {}
        self.doors: Dict[Position, str] = {}
        self.robots: List[Position] = []
        self.collected_keys: Set[str] = set()

        self._populate_grid(input_str)

        self.memo: Dict[Tuple[int, Tuple[Position, ...]], int] = {}

    def _populate_grid(self, input_str: str) -> None:
        '''Populate self.{grid, keys, doors, robots}'''
        starting_pos: Tuple[int, int]
        lines = input_str.rstrip('\n').split('\n')
        for i, line in enumerate(lines):
            self.grid.append([])
            for j, char in enumerate(line):
                if char == '@':
                    starting_pos = (i,j)
                    char = '.'

                self.grid[-1].append(Tile.new(char))

                if self.grid[-1][-1] == Tile.key:
                    self.keys[(i,j)] = char
                elif self.grid[-1][-1] == Tile.door:
                    self.doors[(i,j)] = char

        i, j = starting_pos
        for di, dj in [(-1,0), (0,0), (1,0), (0,-1), (0,1)]:
            self.grid[i+di][j+dj] = Tile.wall
        self.robots = [(i+di,j+dj) for di in (-1,1) for dj in (-1,1)]

    def __str__(self) -> str:
        def to_str(pos: Position, tile: Tile) -> str:
            if pos in self.robots:
                return '@'
            elif pos in self.keys:
                return self.keys[pos]
            elif pos in self.doors:
                return self.doors[pos]
            else:
                return str(tile)

        return '\n'.join(
            ''.join(to_str((i,j), tile) for j, tile in enumerate(row))
            for i, row in enumerate(self.grid)
        ) + '\n'

    def best_route(self) -> int:
        '''
        Return the length of the best path through the maze that collects all
        keys.

        - find reachable keys
        - for each, go pick up that key and then
            recursively find the best route from that state.
        - of all of these, return the best:
            distance to that key PLUS best_route from that key
        - (memoize 'best_route', based on set of collected keys
             together with current position)
        '''
        k = (self.keys_to_bitmask(self.collected_keys), tuple(self.robots))
        if k in self.memo:
            return self.memo[k]

        # We win!
        if len(self.collected_keys) == len(self.keys):
            return 0

        best_dist = 2**62  # Plus infinity.

        # For each robot.
        for i, reachable_keys in enumerate(self.reachable_keys()):
            # For each reachable key.
            for pos, dist in reachable_keys:
                key = self.keys[pos]
                original_pos = self.robots[i]

                # Go collect the key.
                self.collected_keys.add(key)
                self.robots[i] = pos

                # Recurse.
                total_dist = dist + self.best_route()
                best_dist = min(best_dist, total_dist)

                # Backtrack and try all other possible decisions.
                self.collected_keys.remove(key)
                self.robots[i] = original_pos

        self.memo[k] = best_dist
        return best_dist

    @classmethod
    def keys_to_bitmask(cls, keys: Set[str]) -> int:
        '''Convert a subset of [a-z] into a bitmask of bitlength <= 26.
        Low-order bits are abc... high order bits are ...xyz.'''
        bitmask = 0
        for offset, char in enumerate(string.ascii_lowercase):
            if char in keys:
                bitmask |= 1 << offset
        return bitmask

    def reachable_keys(self) -> Iterator[List[Tuple[Position, int]]]:
        '''
        For each robot, return the list of reachable uncollected keys, together
        with each such key's shortest-path-distance from that robot's position.
        '''
        for cur_pos in self.robots:
            reachable = []

            # BFS.
            visited: Set[Position] = set()
            q: Deque[Tuple[Position, int]] = deque()  # position, distance
            q.append((cur_pos, 0))
            while q:
                pos, dist = q.popleft()
                visited.add(pos)

                i, j = pos
                if self.grid[i][j] == Tile.key and \
                    self.keys[pos] not in self.collected_keys:

                    # Pick up the key, but don't pass through it. This is an
                    # optimization which I'm too lazy to explain here.
                    reachable.append((pos, dist))
                    continue

                for pos2 in self.adjacent(pos):
                    x, y = pos2
                    if pos2 not in visited and \
                        self.grid[x][y] != Tile.wall and \
                        (self.grid[x][y] != Tile.door or \
                            self.doors[pos2].lower() in self.collected_keys):

                        q.append((pos2, dist + 1))

            yield reachable

    def adjacent(self, pos: Position) -> Iterator[Position]:
        '''Doesn't check bounds!!'''
        i, j = pos
        for di in -1, 1:
            yield (i+di, j)
        for dj in -1, 1:
            yield (i, j+dj)

if __name__ == '__main__':
    main()
