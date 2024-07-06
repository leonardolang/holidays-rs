
//! Luxembourg
use super::*;

/// Generate holiday map for Luxembourg.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
  let mut map = HashMap::new();

  build_year(
    years,
    2000,
    vec![

      (NaiveDate::from_ymd_res(2000, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2000, 4, 24)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2000, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2000, 6, 1)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2000, 6, 12)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2000, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2000, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2000, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2000, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2000, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2001,
    vec![

      (NaiveDate::from_ymd_res(2001, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2001, 4, 16)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2001, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2001, 5, 24)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2001, 6, 4)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2001, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2001, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2001, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2001, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2001, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2002,
    vec![

      (NaiveDate::from_ymd_res(2002, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2002, 4, 1)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2002, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2002, 5, 9)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2002, 5, 20)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2002, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2002, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2002, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2002, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2002, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2003,
    vec![

      (NaiveDate::from_ymd_res(2003, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2003, 4, 21)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2003, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2003, 5, 29)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2003, 6, 9)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2003, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2003, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2003, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2003, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2003, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2004,
    vec![

      (NaiveDate::from_ymd_res(2004, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2004, 4, 12)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2004, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2004, 5, 20)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2004, 5, 31)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2004, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2004, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2004, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2004, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2004, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2005,
    vec![

      (NaiveDate::from_ymd_res(2005, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2005, 3, 28)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2005, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2005, 5, 5)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2005, 5, 16)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2005, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2005, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2005, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2005, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2005, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2006,
    vec![

      (NaiveDate::from_ymd_res(2006, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2006, 4, 17)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2006, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2006, 5, 25)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2006, 6, 5)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2006, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2006, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2006, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2006, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2006, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2007,
    vec![

      (NaiveDate::from_ymd_res(2007, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2007, 4, 9)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2007, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2007, 5, 17)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2007, 5, 28)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2007, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2007, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2007, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2007, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2007, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2008,
    vec![

      (NaiveDate::from_ymd_res(2008, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2008, 3, 24)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2008, 5, 1)?, "Christi Himmelfaart; Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2008, 5, 12)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2008, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2008, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2008, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2008, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2008, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2009,
    vec![

      (NaiveDate::from_ymd_res(2009, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2009, 4, 13)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2009, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2009, 5, 21)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2009, 6, 1)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2009, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2009, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2009, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2009, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2009, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2010,
    vec![

      (NaiveDate::from_ymd_res(2010, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2010, 4, 5)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2010, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2010, 5, 13)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2010, 5, 24)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2010, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2010, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2010, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2010, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2010, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2011,
    vec![

      (NaiveDate::from_ymd_res(2011, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2011, 4, 25)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2011, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2011, 6, 2)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2011, 6, 13)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2011, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2011, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2011, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2011, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2011, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2012,
    vec![

      (NaiveDate::from_ymd_res(2012, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2012, 4, 9)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2012, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2012, 5, 17)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2012, 5, 28)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2012, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2012, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2012, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2012, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2012, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2013,
    vec![

      (NaiveDate::from_ymd_res(2013, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2013, 4, 1)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2013, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2013, 5, 9)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2013, 5, 20)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2013, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2013, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2013, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2013, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2013, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2014,
    vec![

      (NaiveDate::from_ymd_res(2014, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2014, 4, 21)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2014, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2014, 5, 29)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2014, 6, 9)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2014, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2014, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2014, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2014, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2014, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2015,
    vec![

      (NaiveDate::from_ymd_res(2015, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2015, 4, 6)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2015, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2015, 5, 14)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2015, 5, 25)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2015, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2015, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2015, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2015, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2015, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2016,
    vec![

      (NaiveDate::from_ymd_res(2016, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2016, 3, 28)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2016, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2016, 5, 5)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2016, 5, 16)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2016, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2016, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2016, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2016, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2016, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2017,
    vec![

      (NaiveDate::from_ymd_res(2017, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2017, 4, 17)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2017, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2017, 5, 25)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2017, 6, 5)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2017, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2017, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2017, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2017, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2017, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2018,
    vec![

      (NaiveDate::from_ymd_res(2018, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2018, 4, 2)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2018, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2018, 5, 10)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2018, 5, 21)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2018, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2018, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2018, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2018, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2018, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2019,
    vec![

      (NaiveDate::from_ymd_res(2019, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2019, 4, 22)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2019, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2019, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2019, 5, 30)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2019, 6, 10)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2019, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2019, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2019, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2019, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2019, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2020,
    vec![

      (NaiveDate::from_ymd_res(2020, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2020, 4, 13)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2020, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2020, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2020, 5, 21)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2020, 6, 1)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2020, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2020, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2020, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2020, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2020, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2021,
    vec![

      (NaiveDate::from_ymd_res(2021, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2021, 4, 5)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2021, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2021, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2021, 5, 13)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2021, 5, 24)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2021, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2021, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2021, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2021, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2021, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2022,
    vec![

      (NaiveDate::from_ymd_res(2022, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2022, 4, 18)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2022, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2022, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2022, 5, 26)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2022, 6, 6)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2022, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2022, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2022, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2022, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2022, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2023,
    vec![

      (NaiveDate::from_ymd_res(2023, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2023, 4, 10)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2023, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2023, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2023, 5, 18)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2023, 5, 29)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2023, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2023, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2023, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2023, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2023, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2024,
    vec![

      (NaiveDate::from_ymd_res(2024, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2024, 4, 1)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2024, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2024, 5, 9)?, "Christi Himmelfaart; Europadag"),
      (NaiveDate::from_ymd_res(2024, 5, 20)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2024, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2024, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2024, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2024, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2024, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2025,
    vec![

      (NaiveDate::from_ymd_res(2025, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2025, 4, 21)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2025, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2025, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2025, 5, 29)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2025, 6, 9)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2025, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2025, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2025, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2025, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2025, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2026,
    vec![

      (NaiveDate::from_ymd_res(2026, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2026, 4, 6)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2026, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2026, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2026, 5, 14)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2026, 5, 25)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2026, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2026, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2026, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2026, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2026, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2027,
    vec![

      (NaiveDate::from_ymd_res(2027, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2027, 3, 29)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2027, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2027, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2027, 5, 6)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2027, 5, 17)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2027, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2027, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2027, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2027, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2027, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2028,
    vec![

      (NaiveDate::from_ymd_res(2028, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2028, 4, 17)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2028, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2028, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2028, 5, 25)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2028, 6, 5)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2028, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2028, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2028, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2028, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2028, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2029,
    vec![

      (NaiveDate::from_ymd_res(2029, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2029, 4, 2)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2029, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2029, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2029, 5, 10)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2029, 5, 21)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2029, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2029, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2029, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2029, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2029, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  build_year(
    years,
    2030,
    vec![

      (NaiveDate::from_ymd_res(2030, 1, 1)?, "Neijoerschdag"),
      (NaiveDate::from_ymd_res(2030, 4, 22)?, "Ouschterméindeg"),
      (NaiveDate::from_ymd_res(2030, 5, 1)?, "Dag vun der Aarbecht"),
      (NaiveDate::from_ymd_res(2030, 5, 9)?, "Europadag"),
      (NaiveDate::from_ymd_res(2030, 5, 30)?, "Christi Himmelfaart"),
      (NaiveDate::from_ymd_res(2030, 6, 10)?, "Péngschtméindeg"),
      (NaiveDate::from_ymd_res(2030, 6, 23)?, "Nationalfeierdag"),
      (NaiveDate::from_ymd_res(2030, 8, 15)?, "Léiffrawëschdag"),
      (NaiveDate::from_ymd_res(2030, 11, 1)?, "Allerhellgen"),
      (NaiveDate::from_ymd_res(2030, 12, 25)?, "Chrëschtdag"),
      (NaiveDate::from_ymd_res(2030, 12, 26)?, "Stiefesdag"),
    ],
    &mut map,
    Country::LU,
    "Luxembourg",
  );

  Ok(map)
}