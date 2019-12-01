import re
import sys

def modulus(a: int, b: int) -> int:
    """Aparantly in Python, this really is the modulus operator."""
    return a % b

class Game:
    def __init__(self, num_players: int, last_marble: int) -> None:
        self.num_players = num_players
        self.num_marbles = last_marble + 1

        self.scores = [0 for _ in range(self.num_players)]
        self.next_player = 0

        self.marbles = []
        self.next_marble = 0

        self.curr_marble_idx = None

    def play(self) -> None:
        # Place the initial marble; this doesn't count as a player's turn.
        self.marbles.append(0)
        self.curr_marble_idx = 0
        self.next_marble += 1

        while self.next_marble < self.num_marbles:
            self._play_turn()

    def _play_turn(self) -> None:
        if self.next_marble % 23 != 0:
            # Insert the new marble.
            new_marble_idx = (self.curr_marble_idx + 2) % len(self.marbles)
            self.marbles.insert(new_marble_idx, self.next_marble)
            self.curr_marble_idx = new_marble_idx
        else:  # Edge-case: divisible by 23
            self.scores[self.next_player] += self.next_marble
            bonus_marble_idx = \
                modulus(self.curr_marble_idx - 7, len(self.marbles))
            self.scores[self.next_player] += self.marbles.pop(bonus_marble_idx)
            self.curr_marble_idx = bonus_marble_idx % len(self.marbles)
        self.next_marble += 1
        self.next_player = (self.next_player + 1) % self.num_players

if __name__ == "__main__":
    pattern = r"(\d+) players; last marble is worth (\d+) points"
    num_players, last_marble = \
        map(int, re.match(pattern, sys.stdin.readline()).groups())
    game = Game(num_players, last_marble)
    game.play()
    print(max(game.scores))
