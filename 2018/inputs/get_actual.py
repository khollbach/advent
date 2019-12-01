"""
Ripped from https://gist.github.com/mcpower/87427528b9ba5cac6f0c679370789661
"""

def get_actual(day=None, year=None):
    try:
        actual_input = open("input.txt").read()
        return actual_input
    except FileNotFoundError:
        pass
    from pathlib import Path
    # let's try grabbing it
    search_path = Path(".").resolve()
    try:
        if day is None:
            day = int(search_path.name)
        if year is None:
            year = int(search_path.parent.name)
    except ValueError:
        print("Can't get day and year.")
        print("Backup: save 'input.txt' into the same folder as this script.")
        return ""

    print("{} day {} input not found.".format(year, day))

    # is it time?
    from datetime import datetime, timezone, timedelta
    est = timezone(timedelta(hours=-5))
    unlock_time = datetime(year, 12, day, tzinfo=est)
    cur_time = datetime.now(tz=est)
    delta = unlock_time - cur_time
    if delta.days >= 0:
        print("Remaining time until unlock: {}".format(delta))
        return ""

    while (not list(search_path.glob("*/token.txt"))) and search_path.parent != search_path:
        search_path = search_path.parent

    token_files = list(search_path.glob("*/token.txt"))
    if not token_files:
        assert search_path.parent == search_path
        print("Can't find token.txt in a parent directory.")
        print("Backup: save 'input.txt' into the same folder as this script.")
        return ""

    with token_files[0].open() as f:
        token = f.read().strip()

    # importing requests takes a long time...
    # let's do it without requests.
    import urllib.request
    import urllib.error
    import shutil
    opener = urllib.request.build_opener()
    opener.addheaders = [("Cookie", "session={}".format(token)), ("User-Agent", "python-requests/2.19.1")]
    print("Sending request...")
    url = "https://adventofcode.com/{}/day/{}/input".format(year, day)
    try:
        with opener.open(url) as r:
            with open("input.txt", "wb") as f:
                shutil.copyfileobj(r, f)
            print("Input saved!")
            return open("input.txt").read()
    except urllib.error.HTTPError as e:
        status_code = e.getcode()
        if status_code == 400:
            print("Auth failed!")
        elif status_code == 404:
            print("Day is not out yet????")
        else:
            print("Request failed with code {}??".format(status_code))
        return ""

if __name__ == "__main__":
    get_actual(1, 2018)
