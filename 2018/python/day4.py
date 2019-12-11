import re
import sys

from collections import namedtuple
from enum import Enum, auto

# Part 1

class EventType(Enum):
    BEGIN = auto()
    SLEEP = auto()
    WAKE = auto()

Event = namedtuple('Event', 'id type minute')

class GuardState(Enum):
    AWAKE = auto()
    ASLEEP = auto()

# Read in lines, strip whitespace
lines = []
for line in open('../inputs/4').readlines():
    lines.append(line.strip())

# Sort by the timestamp, which is the first thing on the line.
lines.sort()

# The format of each line.
# We call the last item the 'action'.
line_re = re.compile(r'^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (.*)$')

# Detailed parsing of the action of the line.
begin_re = re.compile(r'^Guard #(\d+) begins shift$')
sleep_str = 'falls asleep'
wake_str = 'wakes up'
def parse_event(last_id, minute, action):
    '''(int or None, int, str) -> Event
    Figure out which type of event this is; throw an exception if it doesn't
    match any of the 3 types we're expecting to see.
    Also throw an exception if last_id is None and the event isn't a BEGIN.

    Create and return an Event object with appropriate fields.
    '''
    match = begin_re.match(action)
    if match:
        return Event(int(match.group(1)), EventType.BEGIN, minute)
    elif action == sleep_str:
        assert last_id is not None
        return Event(last_id, EventType.SLEEP, minute)
    elif action == wake_str:
        assert last_id is not None
        return Event(last_id, EventType.WAKE, minute)
    else:
        raise ValueError('Failed to parse event string.')

events = []
last_id = None
for line in lines:
    minute, action = [f(x) for f, x in zip([int, str],
        line_re.match(line).groups())]
    event = parse_event(last_id, minute, action)
    if event.type == EventType.BEGIN:
        last_id = event.id
    events.append(event)

for i in range(5):
    print(events[i])

print('num events:', len(events))

class NightRecorder:
    def run(self, events):
        '''(, [Event]) -> {int: [[GuardState]]}
        Process the list of events, and return a dictionary mapping each guard
        id to a list of nights on duty.

        Each night is stored as an array of length 60.
        '''
        self.guards = {} # int -> [[GuardState]]

        self.last_id = None
        self.current_night = None
        self.asleep_since = None

        for event in events:
            if event.type == EventType.BEGIN:
                if self.last_id is not None:
                    self.record_night()

                self.init_night(event)
            elif event.type == EventType.SLEEP:
                assert self.last_id is not None
                self.asleep_since = event.minute
            elif event.type == EventType.WAKE:
                assert self.last_id is not None
                assert self.asleep_since is not None
                self.record_nap(event)
            else:
                assert False
        if self.last_id is not None:
            self.record_night()

        return self.guards

    def init_night(self, event):
        self.last_id = event.id
        self.current_night = [GuardState.AWAKE] * 60
        self.asleep_since = None

    def record_nap(self, event):
        '''(, Event or None)'''
        # Mark the relevant time interval as asleep.
        for m in range(self.asleep_since, event.minute if event else 60):
            self.current_night[m] = GuardState.ASLEEP

        self.asleep_since = None

    def record_night(self):
        if self.asleep_since is not None:
            self.record_nap()

        # Append the previous night
        self.guards[self.last_id] = \
                self.guards.get(self.last_id, []) + [self.current_night]

        self.last_id = None
        self.current_night = None

guards = NightRecorder().run(events)

print('guards stats:', len(guards), len(guards[881]), len(guards[881][0]))

most_sleep = -1
best_id = None
for gid in guards:
    sleep = 0
    for night in guards[gid]:
        sleep_indicator = lambda x: 1 if x is GuardState.ASLEEP else 0
        sleep += sum(map(sleep_indicator, night))
    if sleep > most_sleep:
        most_sleep = sleep
        best_id = gid

print('best_id:', best_id, '/ most sleep:', most_sleep)

def most_slept_minute(guard_id, guard):
    minute_to_sleep = [0] * 60
    for night in guard:
        for m, state in enumerate(night):
            if state is GuardState.ASLEEP:
                minute_to_sleep[m] += 1

    most_sleep = -1
    best_minute = None
    for m, sleep_amt in enumerate(minute_to_sleep):
        if sleep_amt > most_sleep:
            most_sleep = sleep_amt
            best_minute = m

    return most_sleep, best_minute, guard_id


# Right answer to part one.
# best_id * best_minute: 140932
print('best minute / most sleep:',
        most_slept_minute(best_id, guards[best_id])[1])
print('best_id * best_minute:',
        best_id * most_slept_minute(best_id, guards[best_id])[1])

# Part 2

print(max(map(lambda x: most_slept_minute(*x), guards.items())))
sleep, minute, gid = max(map(lambda x: most_slept_minute(*x), guards.items()))
print(minute * gid)
