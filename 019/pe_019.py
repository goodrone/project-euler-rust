# ymdow is (year, month, day_of_week)
# month is 0 for January, 1 for February, etc.
# day_of_week is 0 for Sunday, 1 for Monday, etc.

DAYS_IN_MONTH = [31, None, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

def next_month(ymdow):
    m = ymdow[1]
    days_in_month = DAYS_IN_MONTH[m]
    y = ymdow[0]
    if days_in_month is None:
        if y % 4 == 0 and (y % 100 != 0 or y % 400 == 0):
            days_in_month = 29
        else:
            days_in_month = 28
    new_dow = (ymdow[2] + days_in_month) % 7
    new_month = m + 1
    new_year = y
    if new_month >= 12:
        new_year += 1
        new_month = 0
    return (new_year, new_month, new_dow)

def format_ymdow(ymdow):
    y = ymdow[0]
    m = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"][ymdow[1]]
    dow = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday",
        "Saturday"][ymdow[2]]
    return f"{y}, {m} is {dow}"

ymdow = (1900, 0, 1)
count = 0
while True:
    if ymdow[0] >= 2001:
        break
    if ymdow[0] >= 1901 and ymdow[2] == 0:
        count += 1
    ymdow = next_month(ymdow)
print(count)
