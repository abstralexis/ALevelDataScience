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

"""--sql
CREATE TABLE IF NOT EXISTS LocationNames (
    UniqueID INT PRIMARY KEY AUTOINCREMENT,
    LocationName VARCHAR NOT NULL,
);
"""

"""--sql
CREATE TABLE IF NOT EXISTS BeaufortUnits (
    UniqueID INT PRIMARY KEY AUTOINCREMENT,
    UnitName VARCHAR NOT NULL,
);
"""

"""--sql
CREATE TABLE IF NOT EXISTS CardinalDirections (
    UniqueID INT PRIMARY KEY AUTOINCREMENT,
    Direction VARCHAR NOT NULL,    
);
"""

"""--sql
CREATE TABLE IF NOT EXISTS LocalData (
    LocationID INT FOREIGN KEY REFERENCES LocationNames(UniqueID),
    DataYear INT NOT NULL,
    DataMonth INT NOT NULL,
    DataDay INT NOT NULL,
    MeanAirTempC REAL,
    TotalRainfallMm REAL,
    TotalSunshineHrs REAL,
    MeanWindspeedKn INT,
    MeanWindspeedBft INT FOREIGN KEY REFERENCES BeaufortUnits(UniqueID),
    MaxGustKn INT,
    MaxHumidityPercent INT,
    MeanCloudOktas INT,
    MeanVisibilityDm INT,
    MeanPressureHpa INT,
    MeanWindDirDeg INT,
    MeanWindDirCardinal INT FOREIGN KEY REFERENCES CardinalDirections(UniqueID),
    MaxGustDirDeg INT,
    MaxGustDirCardinal INT FOREIGN KEY REFERENCES CardinalDirections(UniqueID),
    PRIMARY KEY (LocationID, DataYear, DataMonth, DataDay),
);
"""

"""--sql
CREATE TABLE IF NOT EXISTS OverseasData (
    LocationID INT FOREIGN KEY REFERENCES LocationNames(UniqueID),
    DataYear INT NOT NULL,
    DataMonth INT NOT NULL,
    DataDay INT NOT NULL,
    MeanAirTempC REAL,
    TotalRainfallMm REAL,
    MeanPressureHpa INT,
    MeanWindspeedKn INT,
    MeanWindspeedBft INT FOREIGN KEY REFERENCES BeaufortUnits(UniqueID),
    PRIMARY KEY (LocationID, DataYear, DataMonth, DataDay),
);
"""  