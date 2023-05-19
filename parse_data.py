import csv

# Important indexes --------------------------------------------
NAME_INDEX = 0
NGR_INDEX = 1
ALTITUDE_INDEX = 2
LATITUDE_INDEX = 3
# blank line
COLUMN_NAMES_INDEX = 5
# Data starts
# --------------------------------------------------------------

with open(
          "data\Large Data Set - Beijing May-Oct 1987.csv",
          "r") as file:
    data = csv.reader(file)
    for index, row in enumerate(data):
        if index in set([NAME_INDEX,
                        NGR_INDEX,
                        ALTITUDE_INDEX,
                        LATITUDE_INDEX]):
            print(row[0])
        elif index == COLUMN_NAMES_INDEX:
            print(row)
            
     