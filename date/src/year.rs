/// # INVARIANT
/// Input must be [u8; 10] with first 4 bytes being the `YEAR`.
pub const fn year(bytes: &[u8]) -> Option<(u16, [u8; 4])> {
	Some(match bytes {
		[49, 57, 48, 48,_,_,_,_,_,_] => (1900, [49, 57, 48, 48]),
		[49, 57, 48, 49,_,_,_,_,_,_] => (1901, [49, 57, 48, 49]),
		[49, 57, 48, 50,_,_,_,_,_,_] => (1902, [49, 57, 48, 50]),
		[49, 57, 48, 51,_,_,_,_,_,_] => (1903, [49, 57, 48, 51]),
		[49, 57, 48, 52,_,_,_,_,_,_] => (1904, [49, 57, 48, 52]),
		[49, 57, 48, 53,_,_,_,_,_,_] => (1905, [49, 57, 48, 53]),
		[49, 57, 48, 54,_,_,_,_,_,_] => (1906, [49, 57, 48, 54]),
		[49, 57, 48, 55,_,_,_,_,_,_] => (1907, [49, 57, 48, 55]),
		[49, 57, 48, 56,_,_,_,_,_,_] => (1908, [49, 57, 48, 56]),
		[49, 57, 48, 57,_,_,_,_,_,_] => (1909, [49, 57, 48, 57]),
		[49, 57, 49, 48,_,_,_,_,_,_] => (1910, [49, 57, 49, 48]),
		[49, 57, 49, 49,_,_,_,_,_,_] => (1911, [49, 57, 49, 49]),
		[49, 57, 49, 50,_,_,_,_,_,_] => (1912, [49, 57, 49, 50]),
		[49, 57, 49, 51,_,_,_,_,_,_] => (1913, [49, 57, 49, 51]),
		[49, 57, 49, 52,_,_,_,_,_,_] => (1914, [49, 57, 49, 52]),
		[49, 57, 49, 53,_,_,_,_,_,_] => (1915, [49, 57, 49, 53]),
		[49, 57, 49, 54,_,_,_,_,_,_] => (1916, [49, 57, 49, 54]),
		[49, 57, 49, 55,_,_,_,_,_,_] => (1917, [49, 57, 49, 55]),
		[49, 57, 49, 56,_,_,_,_,_,_] => (1918, [49, 57, 49, 56]),
		[49, 57, 49, 57,_,_,_,_,_,_] => (1919, [49, 57, 49, 57]),
		[49, 57, 50, 48,_,_,_,_,_,_] => (1920, [49, 57, 50, 48]),
		[49, 57, 50, 49,_,_,_,_,_,_] => (1921, [49, 57, 50, 49]),
		[49, 57, 50, 50,_,_,_,_,_,_] => (1922, [49, 57, 50, 50]),
		[49, 57, 50, 51,_,_,_,_,_,_] => (1923, [49, 57, 50, 51]),
		[49, 57, 50, 52,_,_,_,_,_,_] => (1924, [49, 57, 50, 52]),
		[49, 57, 50, 53,_,_,_,_,_,_] => (1925, [49, 57, 50, 53]),
		[49, 57, 50, 54,_,_,_,_,_,_] => (1926, [49, 57, 50, 54]),
		[49, 57, 50, 55,_,_,_,_,_,_] => (1927, [49, 57, 50, 55]),
		[49, 57, 50, 56,_,_,_,_,_,_] => (1928, [49, 57, 50, 56]),
		[49, 57, 50, 57,_,_,_,_,_,_] => (1929, [49, 57, 50, 57]),
		[49, 57, 51, 48,_,_,_,_,_,_] => (1930, [49, 57, 51, 48]),
		[49, 57, 51, 49,_,_,_,_,_,_] => (1931, [49, 57, 51, 49]),
		[49, 57, 51, 50,_,_,_,_,_,_] => (1932, [49, 57, 51, 50]),
		[49, 57, 51, 51,_,_,_,_,_,_] => (1933, [49, 57, 51, 51]),
		[49, 57, 51, 52,_,_,_,_,_,_] => (1934, [49, 57, 51, 52]),
		[49, 57, 51, 53,_,_,_,_,_,_] => (1935, [49, 57, 51, 53]),
		[49, 57, 51, 54,_,_,_,_,_,_] => (1936, [49, 57, 51, 54]),
		[49, 57, 51, 55,_,_,_,_,_,_] => (1937, [49, 57, 51, 55]),
		[49, 57, 51, 56,_,_,_,_,_,_] => (1938, [49, 57, 51, 56]),
		[49, 57, 51, 57,_,_,_,_,_,_] => (1939, [49, 57, 51, 57]),
		[49, 57, 52, 48,_,_,_,_,_,_] => (1940, [49, 57, 52, 48]),
		[49, 57, 52, 49,_,_,_,_,_,_] => (1941, [49, 57, 52, 49]),
		[49, 57, 52, 50,_,_,_,_,_,_] => (1942, [49, 57, 52, 50]),
		[49, 57, 52, 51,_,_,_,_,_,_] => (1943, [49, 57, 52, 51]),
		[49, 57, 52, 52,_,_,_,_,_,_] => (1944, [49, 57, 52, 52]),
		[49, 57, 52, 53,_,_,_,_,_,_] => (1945, [49, 57, 52, 53]),
		[49, 57, 52, 54,_,_,_,_,_,_] => (1946, [49, 57, 52, 54]),
		[49, 57, 52, 55,_,_,_,_,_,_] => (1947, [49, 57, 52, 55]),
		[49, 57, 52, 56,_,_,_,_,_,_] => (1948, [49, 57, 52, 56]),
		[49, 57, 52, 57,_,_,_,_,_,_] => (1949, [49, 57, 52, 57]),
		[49, 57, 53, 48,_,_,_,_,_,_] => (1950, [49, 57, 53, 48]),
		[49, 57, 53, 49,_,_,_,_,_,_] => (1951, [49, 57, 53, 49]),
		[49, 57, 53, 50,_,_,_,_,_,_] => (1952, [49, 57, 53, 50]),
		[49, 57, 53, 51,_,_,_,_,_,_] => (1953, [49, 57, 53, 51]),
		[49, 57, 53, 52,_,_,_,_,_,_] => (1954, [49, 57, 53, 52]),
		[49, 57, 53, 53,_,_,_,_,_,_] => (1955, [49, 57, 53, 53]),
		[49, 57, 53, 54,_,_,_,_,_,_] => (1956, [49, 57, 53, 54]),
		[49, 57, 53, 55,_,_,_,_,_,_] => (1957, [49, 57, 53, 55]),
		[49, 57, 53, 56,_,_,_,_,_,_] => (1958, [49, 57, 53, 56]),
		[49, 57, 53, 57,_,_,_,_,_,_] => (1959, [49, 57, 53, 57]),
		[49, 57, 54, 48,_,_,_,_,_,_] => (1960, [49, 57, 54, 48]),
		[49, 57, 54, 49,_,_,_,_,_,_] => (1961, [49, 57, 54, 49]),
		[49, 57, 54, 50,_,_,_,_,_,_] => (1962, [49, 57, 54, 50]),
		[49, 57, 54, 51,_,_,_,_,_,_] => (1963, [49, 57, 54, 51]),
		[49, 57, 54, 52,_,_,_,_,_,_] => (1964, [49, 57, 54, 52]),
		[49, 57, 54, 53,_,_,_,_,_,_] => (1965, [49, 57, 54, 53]),
		[49, 57, 54, 54,_,_,_,_,_,_] => (1966, [49, 57, 54, 54]),
		[49, 57, 54, 55,_,_,_,_,_,_] => (1967, [49, 57, 54, 55]),
		[49, 57, 54, 56,_,_,_,_,_,_] => (1968, [49, 57, 54, 56]),
		[49, 57, 54, 57,_,_,_,_,_,_] => (1969, [49, 57, 54, 57]),
		[49, 57, 55, 48,_,_,_,_,_,_] => (1970, [49, 57, 55, 48]),
		[49, 57, 55, 49,_,_,_,_,_,_] => (1971, [49, 57, 55, 49]),
		[49, 57, 55, 50,_,_,_,_,_,_] => (1972, [49, 57, 55, 50]),
		[49, 57, 55, 51,_,_,_,_,_,_] => (1973, [49, 57, 55, 51]),
		[49, 57, 55, 52,_,_,_,_,_,_] => (1974, [49, 57, 55, 52]),
		[49, 57, 55, 53,_,_,_,_,_,_] => (1975, [49, 57, 55, 53]),
		[49, 57, 55, 54,_,_,_,_,_,_] => (1976, [49, 57, 55, 54]),
		[49, 57, 55, 55,_,_,_,_,_,_] => (1977, [49, 57, 55, 55]),
		[49, 57, 55, 56,_,_,_,_,_,_] => (1978, [49, 57, 55, 56]),
		[49, 57, 55, 57,_,_,_,_,_,_] => (1979, [49, 57, 55, 57]),
		[49, 57, 56, 48,_,_,_,_,_,_] => (1980, [49, 57, 56, 48]),
		[49, 57, 56, 49,_,_,_,_,_,_] => (1981, [49, 57, 56, 49]),
		[49, 57, 56, 50,_,_,_,_,_,_] => (1982, [49, 57, 56, 50]),
		[49, 57, 56, 51,_,_,_,_,_,_] => (1983, [49, 57, 56, 51]),
		[49, 57, 56, 52,_,_,_,_,_,_] => (1984, [49, 57, 56, 52]),
		[49, 57, 56, 53,_,_,_,_,_,_] => (1985, [49, 57, 56, 53]),
		[49, 57, 56, 54,_,_,_,_,_,_] => (1986, [49, 57, 56, 54]),
		[49, 57, 56, 55,_,_,_,_,_,_] => (1987, [49, 57, 56, 55]),
		[49, 57, 56, 56,_,_,_,_,_,_] => (1988, [49, 57, 56, 56]),
		[49, 57, 56, 57,_,_,_,_,_,_] => (1989, [49, 57, 56, 57]),
		[49, 57, 57, 48,_,_,_,_,_,_] => (1990, [49, 57, 57, 48]),
		[49, 57, 57, 49,_,_,_,_,_,_] => (1991, [49, 57, 57, 49]),
		[49, 57, 57, 50,_,_,_,_,_,_] => (1992, [49, 57, 57, 50]),
		[49, 57, 57, 51,_,_,_,_,_,_] => (1993, [49, 57, 57, 51]),
		[49, 57, 57, 52,_,_,_,_,_,_] => (1994, [49, 57, 57, 52]),
		[49, 57, 57, 53,_,_,_,_,_,_] => (1995, [49, 57, 57, 53]),
		[49, 57, 57, 54,_,_,_,_,_,_] => (1996, [49, 57, 57, 54]),
		[49, 57, 57, 55,_,_,_,_,_,_] => (1997, [49, 57, 57, 55]),
		[49, 57, 57, 56,_,_,_,_,_,_] => (1998, [49, 57, 57, 56]),
		[49, 57, 57, 57,_,_,_,_,_,_] => (1999, [49, 57, 57, 57]),
		[50, 48, 48, 48,_,_,_,_,_,_] => (2000, [50, 48, 48, 48]),
		[50, 48, 48, 49,_,_,_,_,_,_] => (2001, [50, 48, 48, 49]),
		[50, 48, 48, 50,_,_,_,_,_,_] => (2002, [50, 48, 48, 50]),
		[50, 48, 48, 51,_,_,_,_,_,_] => (2003, [50, 48, 48, 51]),
		[50, 48, 48, 52,_,_,_,_,_,_] => (2004, [50, 48, 48, 52]),
		[50, 48, 48, 53,_,_,_,_,_,_] => (2005, [50, 48, 48, 53]),
		[50, 48, 48, 54,_,_,_,_,_,_] => (2006, [50, 48, 48, 54]),
		[50, 48, 48, 55,_,_,_,_,_,_] => (2007, [50, 48, 48, 55]),
		[50, 48, 48, 56,_,_,_,_,_,_] => (2008, [50, 48, 48, 56]),
		[50, 48, 48, 57,_,_,_,_,_,_] => (2009, [50, 48, 48, 57]),
		[50, 48, 49, 48,_,_,_,_,_,_] => (2010, [50, 48, 49, 48]),
		[50, 48, 49, 49,_,_,_,_,_,_] => (2011, [50, 48, 49, 49]),
		[50, 48, 49, 50,_,_,_,_,_,_] => (2012, [50, 48, 49, 50]),
		[50, 48, 49, 51,_,_,_,_,_,_] => (2013, [50, 48, 49, 51]),
		[50, 48, 49, 52,_,_,_,_,_,_] => (2014, [50, 48, 49, 52]),
		[50, 48, 49, 53,_,_,_,_,_,_] => (2015, [50, 48, 49, 53]),
		[50, 48, 49, 54,_,_,_,_,_,_] => (2016, [50, 48, 49, 54]),
		[50, 48, 49, 55,_,_,_,_,_,_] => (2017, [50, 48, 49, 55]),
		[50, 48, 49, 56,_,_,_,_,_,_] => (2018, [50, 48, 49, 56]),
		[50, 48, 49, 57,_,_,_,_,_,_] => (2019, [50, 48, 49, 57]),
		[50, 48, 50, 48,_,_,_,_,_,_] => (2020, [50, 48, 50, 48]),
		[50, 48, 50, 49,_,_,_,_,_,_] => (2021, [50, 48, 50, 49]),
		[50, 48, 50, 50,_,_,_,_,_,_] => (2022, [50, 48, 50, 50]),
		[50, 48, 50, 51,_,_,_,_,_,_] => (2023, [50, 48, 50, 51]),
		[50, 48, 50, 52,_,_,_,_,_,_] => (2024, [50, 48, 50, 52]),
		[50, 48, 50, 53,_,_,_,_,_,_] => (2025, [50, 48, 50, 53]),
		[50, 48, 50, 54,_,_,_,_,_,_] => (2026, [50, 48, 50, 54]),
		[50, 48, 50, 55,_,_,_,_,_,_] => (2027, [50, 48, 50, 55]),
		[50, 48, 50, 56,_,_,_,_,_,_] => (2028, [50, 48, 50, 56]),
		[50, 48, 50, 57,_,_,_,_,_,_] => (2029, [50, 48, 50, 57]),
		[50, 48, 51, 48,_,_,_,_,_,_] => (2030, [50, 48, 51, 48]),
		[50, 48, 51, 49,_,_,_,_,_,_] => (2031, [50, 48, 51, 49]),
		[50, 48, 51, 50,_,_,_,_,_,_] => (2032, [50, 48, 51, 50]),
		[50, 48, 51, 51,_,_,_,_,_,_] => (2033, [50, 48, 51, 51]),
		[50, 48, 51, 52,_,_,_,_,_,_] => (2034, [50, 48, 51, 52]),
		[50, 48, 51, 53,_,_,_,_,_,_] => (2035, [50, 48, 51, 53]),
		[50, 48, 51, 54,_,_,_,_,_,_] => (2036, [50, 48, 51, 54]),
		[50, 48, 51, 55,_,_,_,_,_,_] => (2037, [50, 48, 51, 55]),
		[50, 48, 51, 56,_,_,_,_,_,_] => (2038, [50, 48, 51, 56]),
		[50, 48, 51, 57,_,_,_,_,_,_] => (2039, [50, 48, 51, 57]),
		[50, 48, 52, 48,_,_,_,_,_,_] => (2040, [50, 48, 52, 48]),
		[50, 48, 52, 49,_,_,_,_,_,_] => (2041, [50, 48, 52, 49]),
		[50, 48, 52, 50,_,_,_,_,_,_] => (2042, [50, 48, 52, 50]),
		[50, 48, 52, 51,_,_,_,_,_,_] => (2043, [50, 48, 52, 51]),
		[50, 48, 52, 52,_,_,_,_,_,_] => (2044, [50, 48, 52, 52]),
		[50, 48, 52, 53,_,_,_,_,_,_] => (2045, [50, 48, 52, 53]),
		[50, 48, 52, 54,_,_,_,_,_,_] => (2046, [50, 48, 52, 54]),
		[50, 48, 52, 55,_,_,_,_,_,_] => (2047, [50, 48, 52, 55]),
		[50, 48, 52, 56,_,_,_,_,_,_] => (2048, [50, 48, 52, 56]),
		[50, 48, 52, 57,_,_,_,_,_,_] => (2049, [50, 48, 52, 57]),
		[50, 48, 53, 48,_,_,_,_,_,_] => (2050, [50, 48, 53, 48]),
		[50, 48, 53, 49,_,_,_,_,_,_] => (2051, [50, 48, 53, 49]),
		[50, 48, 53, 50,_,_,_,_,_,_] => (2052, [50, 48, 53, 50]),
		[50, 48, 53, 51,_,_,_,_,_,_] => (2053, [50, 48, 53, 51]),
		[50, 48, 53, 52,_,_,_,_,_,_] => (2054, [50, 48, 53, 52]),
		[50, 48, 53, 53,_,_,_,_,_,_] => (2055, [50, 48, 53, 53]),
		[50, 48, 53, 54,_,_,_,_,_,_] => (2056, [50, 48, 53, 54]),
		[50, 48, 53, 55,_,_,_,_,_,_] => (2057, [50, 48, 53, 55]),
		[50, 48, 53, 56,_,_,_,_,_,_] => (2058, [50, 48, 53, 56]),
		[50, 48, 53, 57,_,_,_,_,_,_] => (2059, [50, 48, 53, 57]),
		[50, 48, 54, 48,_,_,_,_,_,_] => (2060, [50, 48, 54, 48]),
		[50, 48, 54, 49,_,_,_,_,_,_] => (2061, [50, 48, 54, 49]),
		[50, 48, 54, 50,_,_,_,_,_,_] => (2062, [50, 48, 54, 50]),
		[50, 48, 54, 51,_,_,_,_,_,_] => (2063, [50, 48, 54, 51]),
		[50, 48, 54, 52,_,_,_,_,_,_] => (2064, [50, 48, 54, 52]),
		[50, 48, 54, 53,_,_,_,_,_,_] => (2065, [50, 48, 54, 53]),
		[50, 48, 54, 54,_,_,_,_,_,_] => (2066, [50, 48, 54, 54]),
		[50, 48, 54, 55,_,_,_,_,_,_] => (2067, [50, 48, 54, 55]),
		[50, 48, 54, 56,_,_,_,_,_,_] => (2068, [50, 48, 54, 56]),
		[50, 48, 54, 57,_,_,_,_,_,_] => (2069, [50, 48, 54, 57]),
		[50, 48, 55, 48,_,_,_,_,_,_] => (2070, [50, 48, 55, 48]),
		[50, 48, 55, 49,_,_,_,_,_,_] => (2071, [50, 48, 55, 49]),
		[50, 48, 55, 50,_,_,_,_,_,_] => (2072, [50, 48, 55, 50]),
		[50, 48, 55, 51,_,_,_,_,_,_] => (2073, [50, 48, 55, 51]),
		[50, 48, 55, 52,_,_,_,_,_,_] => (2074, [50, 48, 55, 52]),
		[50, 48, 55, 53,_,_,_,_,_,_] => (2075, [50, 48, 55, 53]),
		[50, 48, 55, 54,_,_,_,_,_,_] => (2076, [50, 48, 55, 54]),
		[50, 48, 55, 55,_,_,_,_,_,_] => (2077, [50, 48, 55, 55]),
		[50, 48, 55, 56,_,_,_,_,_,_] => (2078, [50, 48, 55, 56]),
		[50, 48, 55, 57,_,_,_,_,_,_] => (2079, [50, 48, 55, 57]),
		[50, 48, 56, 48,_,_,_,_,_,_] => (2080, [50, 48, 56, 48]),
		[50, 48, 56, 49,_,_,_,_,_,_] => (2081, [50, 48, 56, 49]),
		[50, 48, 56, 50,_,_,_,_,_,_] => (2082, [50, 48, 56, 50]),
		[50, 48, 56, 51,_,_,_,_,_,_] => (2083, [50, 48, 56, 51]),
		[50, 48, 56, 52,_,_,_,_,_,_] => (2084, [50, 48, 56, 52]),
		[50, 48, 56, 53,_,_,_,_,_,_] => (2085, [50, 48, 56, 53]),
		[50, 48, 56, 54,_,_,_,_,_,_] => (2086, [50, 48, 56, 54]),
		[50, 48, 56, 55,_,_,_,_,_,_] => (2087, [50, 48, 56, 55]),
		[50, 48, 56, 56,_,_,_,_,_,_] => (2088, [50, 48, 56, 56]),
		[50, 48, 56, 57,_,_,_,_,_,_] => (2089, [50, 48, 56, 57]),
		[50, 48, 57, 48,_,_,_,_,_,_] => (2090, [50, 48, 57, 48]),
		[50, 48, 57, 49,_,_,_,_,_,_] => (2091, [50, 48, 57, 49]),
		[50, 48, 57, 50,_,_,_,_,_,_] => (2092, [50, 48, 57, 50]),
		[50, 48, 57, 51,_,_,_,_,_,_] => (2093, [50, 48, 57, 51]),
		[50, 48, 57, 52,_,_,_,_,_,_] => (2094, [50, 48, 57, 52]),
		[50, 48, 57, 53,_,_,_,_,_,_] => (2095, [50, 48, 57, 53]),
		[50, 48, 57, 54,_,_,_,_,_,_] => (2096, [50, 48, 57, 54]),
		[50, 48, 57, 55,_,_,_,_,_,_] => (2097, [50, 48, 57, 55]),
		[50, 48, 57, 56,_,_,_,_,_,_] => (2098, [50, 48, 57, 56]),
		[50, 48, 57, 57,_,_,_,_,_,_] => (2099, [50, 48, 57, 57]),
		[50, 49, 48, 48,_,_,_,_,_,_] => (2100, [50, 49, 48, 48]),
		_ => return None,
	})
}
